use crate::db::client::connect_to_db;
// use mongodb::{bson::doc, bson::Document, Collection};
use tokio;

#[tokio::main]
pub async fn get_products() {
    let connection = connect_to_db().await;



    // for item in documents {
    //     println!("{:?}", item)
    // }

    println!("test ing");
}
