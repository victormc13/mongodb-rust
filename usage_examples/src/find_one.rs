use mongodb::{
    bson::{doc,Document},
    Client,
    Collection,
};
use serde::{ Deserialize, Serialize };
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    username: String,
    name: String,
    addres: String,
    email: String,
    accounts: Vec<i32>,
}

#[tokio::main]
pub async fn find_one() -> mongodb::error::Result<()> {

    // Load .env file
    dotenv().ok();
    // Get the env variable from .env file
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found");

    let client = Client::with_uri_str(mongodb_uri).await?;

    let coll: Collection<Document> = client.database("sample_analytics").collection("customers");

    let result = coll.find_one(doc! {"username": "serranobrian"}, None).await?;

    println!("Found user: {:#?}", result.unwrap());

    Ok(())
}
