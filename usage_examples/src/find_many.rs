use mongodb::{
    bson::{doc, DateTime, Document},
    Client,
    Collection
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
pub async fn find_many() -> mongodb::error::Result<()> {

    // Load .env file
    dotenv().ok();
    // Get the MONGODB_URI env variable
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found");

    let client = Client::with_uri_str(mongodb_uri).await?;

    let coll: Collection<Document> = client.database("sample_analytics").collection("customers");

    let filter = doc! {"name": "Brad Cardenas"};
    let mut cursor = coll.find(filter, None).await?;

    let mut counter = 1;
    println!("Found users:");
    while let Some(doc) = cursor.try_next().await? {
        println!("User {counter}: {:#?}", doc);
        counter += 1;
    }

    Ok(())
}
