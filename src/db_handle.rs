pub mod models;
pub mod validation;

use mongodb::{ options::ClientOptions, Client, Database, Cursor};

use std::env;


pub struct DatabaseHandle {
    pub database: Database
}

impl DatabaseHandle {
    pub async fn new(database_name: &str) -> Self {
        let client_options = ClientOptions::parse(
            env::var("MONGO_URL").expect("Mongo url not exist!")
        ).await.expect("Auth is wrong!");
        let client = Client::with_options(client_options)
            .expect("Client was not created");
        Self {
            database: client.database(database_name)
        }
    }



}

