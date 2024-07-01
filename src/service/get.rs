use mongodb::{
    Database, bson::Document
};
use futures::TryStreamExt;

pub async fn get_products(connect: &Database) -> Result<(), mongodb::error::Error> {
    let collection: mongodb::Collection<Document> = connect.collection("frame_products");
    let mut cursor = collection.find(None, None).await?;
    
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }

    println!("test ing");
    Ok(())
}

