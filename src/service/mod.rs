pub mod get;

use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Product {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub category: Vec<String>,
    pub price: f32,
    pub description: String,
    pub rating_data: Rating
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Rating {
  pub count: i32,
  pub rating: i32
}