use mongodb::bson::{doc, Document};
use mongodb::Collection;
use crate::DatabaseHandle;

impl DatabaseHandle {
    pub async fn value_is_exists(
        collection: Collection<Document>,
        field:&str,
        value:&str
    ) -> bool {
        match collection.find(doc! {field: value}, None).await {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
