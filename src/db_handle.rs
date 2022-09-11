use mongodb::{bson::doc, options::ClientOptions, Client, Database, Collection, Cursor};
use mongodb::bson::Document;
use mongodb::results::{DeleteResult, InsertOneResult};
use std::env;

use crate::clock_handle::{Clock};

pub struct DatabaseHandle {
    database: Database,
    collection: Collection<Document>,
}

impl DatabaseHandle {
    pub async fn new(database_name: &str, current_collection_name: &str) -> Self {
        let client_options = ClientOptions::parse(
            env::var("MONGO_URL").expect("Mongo url not exist!")
        ).await.expect("Auth is wrong!");
        let client = Client::with_options(client_options)
            .expect("Client was not created");
        Self {
            database: client.database(database_name),
            collection: client.database(database_name).collection(current_collection_name)
        }
    }

    pub fn change_collection(&mut self, collection_name: &str) {
        self.collection = self.database.collection(collection_name);
    }

    pub async fn add_telegram_user(&self, user_id: u64, chat_id: i64) -> mongodb::error::Result<InsertOneResult> {
        let document = doc! {
            "type": "user",
            "user_id": user_id.to_string(),
            "chat_id": chat_id.to_string(),
            "creating_datetime": Clock::get_current_datetime().to_string(),
        };

        self.collection.insert_one(document, None).await
    }

    pub async fn get_telegram_users(&self) -> Cursor<Document> {
        let users_cursor: Cursor<Document> = self.collection.find(
            doc! {"type":"user"},
            None
        ).await.unwrap();
        users_cursor
    }

}

