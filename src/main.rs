use rocket::http::Status;
use rocket::serde::json::{json, Value};
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
    // service::get::get_products();
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

#[launch]
async fn rocket() -> _ {
    let connection = db::client::connect_to_db().await;
    rocket::build()
        .manage(connection)
        .mount("/", routes![index])
        .mount("/images", routes![all_images, get_image])
}
