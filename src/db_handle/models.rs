use chrono::{NaiveDateTime, NaiveTime};
use serde::{self, Serialize, Deserialize};
use mongodb::bson::{DateTime, doc, Document, from_document};
use mongodb::{Collection, Cursor, Database};
use mongodb::options::{FindOptions, UpdateModifications};

use crate::clock_handle::{ Clock };
use crate::DatabaseHandle;

const COLLECTION_NAME: &str = "telegram_users";

#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct TelegramUser {
    pub user_id: String,
    pub chosen_languages: Vec<String>,
    word_update_interval: String,
    prev_word_update_datetime: String,
    pub next_word_update_datetime: String,
    creating_datetime: String,
}

impl TelegramUser {
    pub async fn new(
        database: Database,
        user_id: String,
    ) -> Option<Self> {
        let collection:Collection<Document> = database.collection(COLLECTION_NAME);
        if !DatabaseHandle::value_is_exists(collection.clone(), "user_id", &user_id).await {
            let model = Self {
                user_id,
                chosen_languages: Vec::new(),
                word_update_interval: NaiveTime::from_hms(0,0,10).to_string(),
                prev_word_update_datetime: Clock::get_current_datetime().to_string(),
                next_word_update_datetime: Clock::add_interval_time_to_time(
                    Clock::get_current_datetime(),
                    NaiveTime::from_hms(0,0,10)
                ).to_string(),
                creating_datetime: Clock::get_current_datetime().to_string()
            };
            collection.clone_with_type().insert_one(doc! {
                "user_id": &model.user_id,
                "chosen_languages": &model.chosen_languages,
                "word_update_interval": &model.word_update_interval.to_string(),
                "prev_word_update_datetime": &model.prev_word_update_datetime.to_string(),
                "next_word_update_datetime": &model.next_word_update_datetime.to_string(),
                "creating_datetime": &model.creating_datetime.to_string()
            }, None).await.expect("Error creating telegram user document");
            Some(model)
        } else {
            None
        }
    }

    pub async fn get_all(database: Database) -> Vec<TelegramUser> {
        let mut all_users:Vec<TelegramUser> = Vec::new();
        let collection:Collection<Document> = database.collection(COLLECTION_NAME);

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
        let mut users = TelegramUser::get_all(database).await;
        if users.len() > 0 {
            users.sort_by(|a,b|
                a.next_word_update_datetime.cmp(&b.next_word_update_datetime)
            );
            Some(users[0].clone())
        } else {
            None
        }

    }

    pub async fn change_next_word_update_datetime(&mut self, database: Database) {
        let collection:Collection<Document> = database.collection(COLLECTION_NAME);
        self.prev_word_update_datetime = Clock::get_current_datetime().to_string();
        self.next_word_update_datetime = Clock::add_interval_time_to_time(
            Clock::get_current_datetime(),
            self.word_update_interval.parse().unwrap()
        ).to_string();
        let update_modifications = UpdateModifications::from(
            doc!{"prev_word_update_datetime": self.prev_word_update_datetime.to_string(),
                "next_word_update_datetime": self.next_word_update_datetime.to_string()}
        );
        collection.find_one_and_update(
            doc! {"user_id": self.user_id.clone()},
            update_modifications,
            None
        ).await.expect("Error find and update");

    }

    pub async fn add(database: Database, user_id: u64) {
        TelegramUser::new(database.clone(), user_id.to_string()).await;
    }
}