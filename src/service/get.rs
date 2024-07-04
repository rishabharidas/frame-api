use bson::doc;
use futures::TryStreamExt;
use mongodb::{
    bson::{oid::ObjectId, Document},
    Database,
};

use crate::service::ProductJson;

use super::Rating;

fn generate_product_data(doc: Document) -> ProductJson {
    let mut product_data = ProductJson {
        id: String::new(),
        name: String::new(),
        description: String::new(),
        price: String::new(),
        rating_data: Rating {
            count: 0,
            rating: 0,
        },
        category: Vec::new(),
    };
    product_data.id = doc.get_object_id("_id").unwrap().to_string();
    product_data.name = doc.get_str("name").unwrap_or("").to_string();
    product_data.description = doc.get_str("description").unwrap_or("").to_string();
    product_data.price = doc.get_str("price").unwrap_or("").to_string();
    product_data.rating_data = Rating {
        count: doc.get_i64("rating_count").unwrap_or(0),
        rating: doc.get_i64("rating").unwrap_or(0),
    };
    product_data.category = doc
        .get_array("category")
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|c| c.as_str().map(|s| s.to_string()))
        .collect();
    product_data
}

pub async fn get_products(connect: &Database) -> Result<Vec<ProductJson>, mongodb::error::Error> {
    let mut products: Vec<ProductJson> = Vec::new();
    let collection: mongodb::Collection<Document> = connect.collection("frame_products");
    let mut cursor = collection.find(None, None).await?;

    while let Some(doc) = cursor.try_next().await? {
        products.push(generate_product_data(doc));
    }
    Ok(products)
}

pub async fn get_product_info(
    connection: &Database,
    id: &str,
) -> Result<ProductJson, mongodb::error::Error> {
    let collection: mongodb::Collection<Document> = connection.collection("frame_products");
    let object_id = ObjectId::parse_str(id)
        .map_err(|e| bson::oid::Error::from(e))
        .unwrap();

    let filter = doc! { "_id": object_id };

    let cursor = collection.find_one(filter, None).await?.unwrap();

    Ok(generate_product_data(cursor))
}
