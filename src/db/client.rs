use dotenv::dotenv;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};
use std::env;
use urlencoding::encode;

pub async fn connect_to_db() -> mongodb::error::Result<Database> {
    dotenv().ok();
    let mongo_password = env::var("MONGO_PASS").expect("password env available not found");
    let mongo_user = env::var("MONGO_USER").expect("user env available not found");
    let mongo_app_name = env::var("MONGO_APP_NAME").expect("mongo app name not found");
    let mongo_domain = env::var("MONGO_DOMAIN").expect("mongo domain not found");

    let uri = format!(
        "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName={}",
        encode(&mongo_user),
        encode(&mongo_password),
        encode(&mongo_domain),
        encode(&mongo_app_name)
    );
    let mut client_options = ClientOptions::parse(&uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    client
        .database("frame_data")
        .run_command(doc! { "ping": 1 }, None)
        .await?;
    let connection = client.database("frame_data");

    println!("Pinged your deployment.");

    Ok(connection)
}
