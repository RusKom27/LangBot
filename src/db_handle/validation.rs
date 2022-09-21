use mongodb::bson::{doc, Document};
use mongodb::{Collection, Cursor, Database};
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseValidation {
    async fn value_is_exists(collection: Collection<Document>, field:&str, value:&str) -> bool;
}

#[async_trait]
impl DatabaseValidation for Database {
    async fn value_is_exists(collection: Collection<Document>, field:&str, value:&str) -> bool {
        collection.find(doc! {field: value}, None).await
            .expect("Find error!").advance().await
            .expect("Advance error!")
    }
}
