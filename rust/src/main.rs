use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{Client, Error};

#[::tokio::main]
async fn main() -> Result<(), aws_sdk_secretsmanager::Error> {
    let secret_name = "rds!db-6301b7a7-8fdb-4c7a-9c37-5dc5033a6ec1";
    let region = Region::new("ap-northeast-1");

    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .region(region)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);

    let response = asm
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    // For a list of exceptions thrown, see
    // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

    let secret_string = response.secret_string();

    // Your code goes here

    Ok(())
}