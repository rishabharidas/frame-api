use bson::{oid::ObjectId, Bson};
use futures::TryStreamExt;
use mongodb::{bson::Document, Database};

use crate::service::ProductInfo;

use super::Rating;

pub async fn get_products(connect: &Database) -> Result<Vec<ProductInfo>, mongodb::error::Error> {
    let mut products: Vec<ProductInfo> = Vec::new();
    let collection: mongodb::Collection<Document> = connect.collection("frame_products");
    let mut cursor = collection.find(None, None).await?;

    while let Some(doc) = cursor.try_next().await? {
        let mut product_data = ProductInfo {
            _id: ObjectId::new(),
            name: String::new(),
            description: String::new(),
            price: String::new(),
            rating_data: Rating {
                count: 0,
                rating: 0,
            },
            category: Vec::new(),
        };
        product_data._id = doc
            .get("_id")
            .expect("cannot find key")
            .as_object_id()
            .unwrap();
        product_data.name = doc
            .get("name")
            .expect("cannot find key")
            .as_str()
            .unwrap()
            .to_string();
        product_data.description = doc
            .get("description")
            .expect("description not found")
            .as_str()
            .unwrap()
            .to_string();
        product_data.price = doc
            .get("price")
            .expect("price not found")
            .as_str()
            .unwrap()
            .to_string();
        product_data.category = match doc.get("category") {
            Some(Bson::Array(arr)) => arr
                .iter()
                .map(|item| {
                    item.as_str()
                        .expect("Category item must be a string")
                        .to_string()
                })
                .collect::<Vec<_>>(),
            None => Vec::new(),
            _ => Vec::new(),
        };
        let rating = doc
            .get("rating_data")
            .expect("ratings key not found")
            .as_document()
            .unwrap();

        product_data.rating_data.count = rating
            .get("count")
            .expect("count key not found")
            .as_i64()
            .unwrap();
        product_data.rating_data.rating = rating
            .get("rating")
            .expect("rating key not found")
            .as_i64()
            .unwrap();
        products.push(product_data);
    }
    Ok(products)
}
