pub mod get;

use mongodb::bson::serde_helpers::serialize_hex_string_as_object_id;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductJson {
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,
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
