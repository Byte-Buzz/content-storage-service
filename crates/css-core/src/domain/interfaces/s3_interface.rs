#[async_trait::async_trait(?Send)]
pub trait S3Interface: Send + Sync {
    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync>;
}
