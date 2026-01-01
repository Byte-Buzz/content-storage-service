use aws_sdk_s3::primitives::ByteStream;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{
    domain::{
        errors::RepositoryError,
        interfaces::S3Interface,
        models::{FileResponseFile, UploadFileRequest},
    },
    infrastructure::{config::S3Config, storage::S3Client},
};

const CHUNK_SIZE: u64 = 8 * 1024 * 1024; // 8 Mebibytes

pub struct S3Repository {
    s3_client: S3Client,
    permanent_bucket: String,
    temp_bucket: String,
}

impl S3Repository {
    pub fn new(s3_client: S3Client, config: &S3Config) -> Self {
        Self {
            s3_client,
            permanent_bucket: config.bucket.clone(),
            temp_bucket: config.temp_bucket.clone(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl S3Interface for S3Repository {
    async fn upload_file(
        &self,
        request: UploadFileRequest<'_>,
    ) -> Result<Option<String>, RepositoryError> {
        self.upload_to_s3(&self.permanent_bucket, request).await
    }

    async fn upload_temp_file(
        &self,
        request: UploadFileRequest<'_>,
    ) -> Result<Option<String>, RepositoryError> {
        self.upload_to_s3(&self.temp_bucket, request).await
    }

    async fn get_file(&self, key: &str) -> Result<FileResponseFile, RepositoryError> {
        let obj = self
            .s3_client
            .get_object()
            .bucket(&self.permanent_bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| RepositoryError::Internal(e.to_string()))?;

        Ok(FileResponseFile {
            file: Box::new(obj.body.into_async_read()),
            content_type: obj.content_type.unwrap_or_default(),
            e_tag: obj.e_tag.unwrap_or_default(),
            filename: String::new(),
            size: obj.content_length.map(|v| v as u32),
        })
    }

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync> {
        Box::new(S3Repository {
            s3_client: self.s3_client.clone(),
            permanent_bucket: self.permanent_bucket.clone(),
            temp_bucket: self.temp_bucket.clone(),
        })
    }
}

impl S3Repository {
    async fn upload_to_s3(
        &self,
        bucket: &str,
        request: UploadFileRequest<'_>,
    ) -> Result<Option<String>, RepositoryError> {
        tracing::info!("uploading file to s3");

        let file = request.file;
        let key = request.key;

        let first_chunk = self
            .read_chunk(file)
            .await
            .map_err(|e| RepositoryError::Io(e))?;

        tracing::debug!("first chunk: {}", first_chunk.len());

        if first_chunk.len() < CHUNK_SIZE as usize {
            tracing::debug!("uploading small file");

            let put_object = self
                .s3_client
                .put_object()
                .bucket(bucket)
                .key(key)
                .content_type(request.content_type)
                .body(ByteStream::from(first_chunk))
                .send()
                .await
                .map_err(|e| RepositoryError::Internal(e.to_string()))?;

            return Ok(put_object.e_tag().map(|s| s.to_string()));
        }

        tracing::debug!("uploading large file");

        let multipart_upload = self
            .s3_client
            .create_multipart_upload()
            .bucket(bucket)
            .key(&key)
            .content_type(request.content_type)
            .send()
            .await
            .map_err(|e| RepositoryError::Internal(e.to_string()))?;
        let upload_id = multipart_upload
            .upload_id
            .ok_or(RepositoryError::Internal("upload id not found".to_string()))?;

        tracing::debug!("upload id: {}", upload_id);

        let mut parts: Vec<aws_sdk_s3::types::CompletedPart> = vec![];
        let mut file_size = first_chunk.len();

        loop {
            let chunk = if parts.len() == 0 {
                first_chunk.clone()
            } else {
                match self.read_chunk(file).await {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(RepositoryError::Io(e));
                    }
                }
            };
            file_size += chunk.len();

            tracing::debug!("file size: {}", file_size);

            let done = chunk.len() < CHUNK_SIZE as usize;

            let part_number = parts.len() as i32 + 1;
            tracing::debug!("uploading part {} with size {}", part_number, chunk.len());
            let part = self
                .s3_client
                .upload_part()
                .bucket(bucket)
                .key(&key)
                .part_number(part_number)
                .upload_id(&upload_id)
                .body(ByteStream::from(chunk))
                .send()
                .await
                .map_err(|e| RepositoryError::Internal(e.to_string()))?;
            let part = aws_sdk_s3::types::CompletedPart::builder()
                .e_tag(
                    part.e_tag()
                        .ok_or(RepositoryError::Internal("e_tag not found".to_string()))?,
                )
                .part_number(part_number)
                .build();
            parts.push(part);

            if done {
                break;
            }
        }

        let completed_multipart_upload = aws_sdk_s3::types::CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();
        let completed_multipart_upload = self
            .s3_client
            .complete_multipart_upload()
            .bucket(bucket)
            .key(key)
            .upload_id(&upload_id)
            .multipart_upload(completed_multipart_upload)
            .send()
            .await
            .map_err(|e| RepositoryError::Internal(e.to_string()))?;

        tracing::info!("finished uploading file to s3");

        Ok(completed_multipart_upload.e_tag().map(|s| s.to_string()))
    }

    async fn read_chunk(&self, file: &mut (dyn AsyncRead + Unpin)) -> std::io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        file.take(CHUNK_SIZE).read_to_end(&mut buffer).await?;
        Ok(buffer)
    }
}
