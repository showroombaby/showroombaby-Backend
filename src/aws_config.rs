use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_s3::Client as S3Client;
use aws_config::meta::region::RegionProviderChain;

pub async fn initialize_aws() -> (DynamoDbClient, S3Client) {
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    
    let dynamodb_client = DynamoDbClient::new(&config);
    let s3_client = S3Client::new(&config);

    (dynamodb_client, s3_client)
}
