use chrono::{NaiveDateTime};
use serde::{self, Serialize, Deserialize};
use mongodb::bson::{DateTime, doc, Document, from_document};
use mongodb::{Collection, Cursor, Database};
use mongodb::options::FindOptions;

use crate::clock_handle::{ Clock };
use crate::DatabaseHandle;

#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct TelegramUser {
    pub user_id: String,
    pub chosen_languages: Vec<String>,
    pub word_update_interval: u32,
    last_word_update_datetime: String,
    creating_datetime: String,
}

impl TelegramUser {
    pub async fn new(
        database: Database,
        user_id: String,
    ) -> Option<Self> {
        let collection:Collection<Document> = database.collection("telegram_users");
        if DatabaseHandle::value_is_exists(collection.clone(), "user_id", &user_id).await {
            let model = Self {
                user_id,
                chosen_languages: Vec::new(),
                word_update_interval: 10,
                last_word_update_datetime: Clock::get_current_datetime().to_string(),
                creating_datetime: Clock::get_current_datetime().to_string()
            };
            collection.clone_with_type().insert_one(doc! {
                "user_id": &model.user_id,
                "chosen_languages": &model.chosen_languages,
                "word_update_interval": &model.word_update_interval.to_string(),
                "last_word_update_datetime": &model.last_word_update_datetime.to_string(),
                "creating_datetime": &model.creating_datetime.to_string()
            }, None).await.expect("Error creating telegram user document");
            Some(model)
        } else {
            None
        }
    }

    pub async fn get_all(database: Database) -> Vec<TelegramUser> {
        let mut all_users:Vec<TelegramUser> = Vec::new();
        let collection:Collection<Document> = database.collection("telegram_users");

        let mut users_cursor: Cursor<Document> = collection.find(
            None,
            None
        ).await.unwrap();

        while users_cursor.advance().await.expect("Get users error") {
            let user:TelegramUser = from_document(
                users_cursor.deserialize_current().expect("Users cursor deserialization error!")
            ).expect("Get data from user document error!");
            all_users.push(user);
        }

        all_users
    }

    pub async fn get_with_closer_update(database: Database) -> Option<TelegramUser> {
        let collection:Collection<Document> = database.collection("telegram_users");

        let find_options = FindOptions::builder()
            .sort(doc!{"word_update_interval": 1})
            .build();

        let mut users_cursor: Cursor<Document> = collection.find(
            None,
            find_options
        ).await.unwrap();

        while users_cursor.advance().await.expect("Get users error") {
            let user:TelegramUser = from_document(
                users_cursor.deserialize_current().expect("Users cursor deserialization error!")
            ).expect("Get data from user document error!");
            return Some(user)
        }
        None
    }

    pub async fn change_last_word_update(database: Database, user: TelegramUser) {

    }

    pub async fn add(database: Database, user_id: u64) {
        TelegramUser::new(database.clone(), user_id.to_string()).await;
    }
}