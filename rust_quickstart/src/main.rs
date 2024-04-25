use mongodb::{
        bson::{Document, doc},
        error::Error,
        Client,
        Collection
};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {

    // Load .env file where is environment variable
    dotenv().ok();
    // Get the env variable from .env file
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI env variable not found");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(mongodb_uri).await?;

    // Get a handle on the database
    let database = client.database("sample_analytics");
    let my_coll: Collection<Document> = database.collection("customers");

    // Find a movie based on the title value
    let my_user = my_coll.find_one(doc! {"username": "hillrachel"}, None).await?;

    // Print the document
    println!("Found a user: \n{:#?}", my_user.unwrap());
    Ok(())
}
