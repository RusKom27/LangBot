use mongodb::bson::{doc, Document};
use mongodb::{Collection, Cursor};
use crate::DatabaseHandle;

impl DatabaseHandle {
    pub async fn value_is_exists(collection: Collection<Document>, field:&str, value:&str) -> bool {
        collection.find(doc! {field: value}, None).await
            .expect("Find error!").advance().await
            .expect("Advance error!")
    }
}
