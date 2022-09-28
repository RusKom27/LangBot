pub mod models;
pub mod validation;

use mongodb::{ options::ClientOptions, Client, Database};
use async_trait::async_trait;
use std::env;

#[async_trait]
pub trait DatabaseHandle {
    async fn connect(database_name: &str) -> Database;
}

#[async_trait]
impl DatabaseHandle for Database {
    async fn connect(database_name: &str) -> Database {
        let client_options = ClientOptions::parse(
            env::var("MONGO_URL").expect("Mongo url not exist!")
        ).await.expect("Auth is wrong!");
        Client::with_options(client_options)
            .expect("Client was not created")
            .database(database_name)
    }
}

