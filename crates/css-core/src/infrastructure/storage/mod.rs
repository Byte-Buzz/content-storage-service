mod post_policy;
mod s3;

pub use post_policy::sign_post_policy;
pub use s3::create_s3_client;

pub type S3Client = aws_sdk_s3::Client;
