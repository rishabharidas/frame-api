use bson::Document;
use mongodb::{self, Database};
use rocket::{serde::json::Json, State};

use super::{ProductBody, Rating};

pub async fn add_product(
    connection: &State<Database>,
    body: Json<ProductBody>,
) -> Result<Result<mongodb::results::InsertOneResult, mongodb::error::Error>, mongodb::error::Error>
{
    let collection: mongodb::Collection<Document> = connection.collection("frame_products");
    let insert_data = ProductBody {
        name: body.name.to_string(),
        category: body.category.to_vec(),
        description: body.description.to_string(),
        rating_data: Rating {
            count: body.rating_data.count,
            rating: body.rating_data.rating,
        },
        price: body.price.to_string(),
    };
    let product_doc = bson::to_document(&insert_data)?;

    let insert_result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(product_doc, None).await;
    Ok(insert_result)
}
