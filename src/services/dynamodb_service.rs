use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::model::AttributeValue;
use crate::db::establish_dynamodb_connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub category: String,
}

pub async fn add_offer(offer: Offer) -> Result<(), Error> {
    let client = establish_dynamodb_connection().await?;

    let request = client.put_item()
        .table_name("showroombaby-offers")
        .item("id", AttributeValue::S(offer.id))
        .item("title", AttributeValue::S(offer.title))
        .item("description", AttributeValue::S(offer.description))
        .item("price", AttributeValue::N(offer.price.to_string()))
        .item("category", AttributeValue::S(offer.category))
        .send()
        .await;

    match request {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
