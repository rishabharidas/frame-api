use mongodb::Database;
use rocket::{
    http::Status,
    serde::json::{self, json, Value},
    State,
};

use service::ProductBody;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate rocket;

mod db;
mod service;

fn get_file_data() -> Value {
    let mut file = File::open("static/images.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannt read file");
    let data: Value = serde_json::from_str(&contents).expect("error occured");
    data
}

#[get("/")]
fn index() -> String {
    String::from("Hello World")
}

#[get("/")]
async fn all_images() -> Result<Value, Status> {
    let images_data = get_file_data();
    Ok(serde_json::json!({"images": images_data}))
}

#[get("/<_id>")]
fn get_image(_id: usize) -> Result<Value, Status> {
    let images_data = get_file_data();
    Ok(json!({
        "url": images_data[_id]
    }))
}

#[get("/")]
async fn all_products(connection: &State<Database>) -> Result<Value, Status> {
    let products = service::get::get_products(connection).await;
    Ok(json!({
        "products" :products.unwrap()
    }))
}
#[get("/<id>")]
async fn get_product(connection: &State<Database>, id: &str) -> Result<Value, Status> {
    let product_detail = service::get::get_product_info(connection, id).await;
    Ok(json!(product_detail.unwrap()))
}

#[post("/", format = "json", data = "<body>")]
async fn insert_product(
    connection: &State<Database>,
    body: json::Json<ProductBody>,
) -> Result<Value, Status> {
    let insert_reponse = service::post::add_product(connection, body).await.unwrap();
    Ok(json!(insert_reponse.unwrap().inserted_id))
}

#[launch]
async fn rocket() -> _ {
    let connection = db::client::connect_to_db().await.unwrap();

    rocket::build()
        .manage(connection)
        .mount("/", routes![index])
        .mount("/images", routes![all_images, get_image])
        .mount(
            "/products",
            routes![all_products, get_product, insert_product],
        )
}
