use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_bucket::HeadBucketError;

pub async fn upload_file(client: &S3Client, bucket: &str, key: &str, body: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    client.put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(body))
        .send()
        .await?;

    Ok(())
}

pub async fn download_file(client: &S3Client, bucket: &str, key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let result = client.get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    let data = result.body.collect().await?;
    Ok(data.into_bytes().to_vec())
}

pub async fn check_bucket_exists(client: &S3Client, bucket: &str) -> Result<bool, SdkError<HeadBucketError>> {
    match client.head_bucket().bucket(bucket).send().await {
        Ok(_) => Ok(true),
        Err(e) => {
            if matches!(e, SdkError::ServiceError(ref err) if err.err().is_not_found()) {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}
