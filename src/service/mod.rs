pub mod get;

use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductInfo {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub name: String,
    pub category: Vec<String>,
    pub price: String,
    pub description: String,
    pub rating_data: Rating,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Rating {
    pub count: i64,
    pub rating: i64,
}
