use std::fmt::Error;
use mongodb::bson::{doc, Document};
use mongodb::{Collection, Cursor, Database};
use async_trait::async_trait;
use crate::db_handle::models::TelegramUser;

#[async_trait]
pub trait DatabaseValidation {
    async fn value_is_exists(collection: Collection<TelegramUser>, field:&str, value:&str) -> Option<TelegramUser>;
}

#[async_trait]
impl DatabaseValidation for Database {
    async fn value_is_exists(collection: Collection<TelegramUser>, field:&str, value:&str) -> Option<TelegramUser> {
        collection.find_one(doc! {field: value}, None).await
            .expect("Find error!")
    }
}
