use crate::infrastructure::{config::Config, storage::create_s3_client};

mod infrastructure;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::from_env()?;
    let s3_client = create_s3_client(&config.s3).await;

    println!("{:?}", s3_client);

    Ok(())
}
