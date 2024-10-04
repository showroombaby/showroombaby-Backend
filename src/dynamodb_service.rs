use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use aws_sdk_dynamodb::Error;

pub async fn create_item(client: &DynamoDbClient, table_name: &str, item: HashMap<String, AttributeValue>) -> Result<(), Box<dyn std::error::Error>> {
    client.put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await?;

    Ok(())
}

pub async fn get_item(client: &DynamoDbClient, table_name: &str, key: HashMap<String, AttributeValue>) -> Result<Option<HashMap<String, AttributeValue>>, Box<dyn std::error::Error>> {
    let result = client.get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await?;

    Ok(result.item)
}

pub async fn test_dynamodb_connection(client: &DynamoDbClient) -> Result<(), Error> {
    client.list_tables().send().await?;
    println!("La connexion à DynamoDB fonctionne !");
    Ok(())
}
