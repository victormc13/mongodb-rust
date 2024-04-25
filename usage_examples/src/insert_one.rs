use mongodb::{
    bson::{doc, Document},
    error::Result,
    Client,
    Collection,
};
use chrono::{Local};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    username: String,
    name: String,
    address: String,
    birthdate: String,
    email: String,
}

#[tokio::main]
pub async fn insert_one() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get connection string from environment variable with error handling
    let uri = env::var("MONGODB_URI").expect("MONODB_URI not found in .env file");

    let client = Client::with_uri_str(&uri).await?;
    let coll: Collection<Customer> = client
        .database("sample_analytics")
        .collection("customers");

    // Get the current local time
    let local_datetime = Local::now();
    // Convert to iso8601 string format
    let iso8601_string: String = local_datetime.to_rfc3339();
    println!("Date and Time of user created: {:?}", iso8601_string.to_string());

    let doc = Customer {
        username: "albertwesker7".to_string(),
        name: "Albert".to_string(),
        address: "West street 3045 CA.".to_string(),
        birthdate: iso8601_string.to_string(),
        email: "alberwesker3421@gmail.com".to_string(),
    };

    let result = coll.insert_one(doc, None).await?;
    println!("Inserted a document with _id: {}", result.inserted_id);


    Ok(())
}

