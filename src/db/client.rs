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

    let uri = format!(
        "mongodb+srv://{}:{}@frame-server.drkkkcv.mongodb.net/?retryWrites=true&w=majority&appName=frame-server",
        encode(&mongo_user),
        encode(&mongo_password)
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
