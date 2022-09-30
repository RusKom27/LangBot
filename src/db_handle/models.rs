use chrono::{NaiveTime};
use frankenstein::Message;
use serde::{self, Serialize, Deserialize};
use mongodb::bson::{doc, Document, from_document};
use mongodb::{Collection, Cursor, Database};
use mongodb::options::{UpdateModifications};

use crate::clock_handle::{ Clock };
use crate::db_handle::validation::DatabaseValidation;
use crate::state_handle::{UserState};
use crate::state_handle::UserState::Idle;
use crate::translator_handle::Translator;

const COLLECTION_NAME: &str = "telegram_users";

#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct TelegramUser {
    pub user_id: String,
    pub chosen_languages: Vec<String>,
    word_update_interval: String,
    prev_word_update_datetime: String,
    pub next_word_update_datetime: String,
    pub current_state: UserState,
    creating_datetime: String,
}

impl TelegramUser {
    pub async fn new(database: Database, user_id: String) -> Option<Self> {
        let collection: Collection<TelegramUser> = database.collection(COLLECTION_NAME);
        match Database::value_is_exists(collection.clone(), "user_id", &user_id).await {
            Some(user) => return Some(user),
            None => {
                let model = Self {
                    user_id,
                    chosen_languages: Vec::new(),
                    word_update_interval: NaiveTime::from_hms(10, 0, 0).to_string(),
                    prev_word_update_datetime: Clock::get_current_datetime().format("%Y-%m-%d %H:%M:%S").to_string(),
                    next_word_update_datetime: Clock::add_interval_time_to_time(
                        Clock::get_current_datetime(),
                        NaiveTime::from_hms(10, 0, 0)
                    ).format("%Y-%m-%d %H:%M:%S").to_string(),
                    current_state: UserState::Idle,
                    creating_datetime: Clock::get_current_datetime().format("%Y-%m-%d %H:%M:%S").to_string()
                };
                collection.clone_with_type().insert_one(doc! {
                    "user_id": &model.user_id,
                    "chosen_languages": &model.chosen_languages,
                    "word_update_interval": &model.word_update_interval,
                    "prev_word_update_datetime": &model.prev_word_update_datetime,
                    "next_word_update_datetime": &model.next_word_update_datetime,
                    "current_state": &model.current_state.to_string(),
                    "creating_datetime": &model.creating_datetime
                }, None).await.expect("Error creating telegram user document");
                return Some(model)
            }
        }
    }

    pub async fn change_state(&mut self, state: UserState, database: Database) -> Option<String> {
        let collection:Collection<TelegramUser> = database.collection(COLLECTION_NAME);
        self.current_state = state;
        self.change_field(&collection, "current_state", self.current_state.to_string()).await;
        match self.current_state {
            UserState::Idle => None,
            UserState::IntervalChanging => None,
            UserState::LanguageChanging => None,
        }
    }

    pub async fn change_params(&mut self, message: &Message, database: Database) -> Option<String> {
        let collection:Collection<TelegramUser> = database.collection(COLLECTION_NAME);
        match self.current_state {
            UserState::Idle => {
                Translator::new().translate_text(&message.clone().text.unwrap(), "uk").await
            }
            UserState::IntervalChanging => {
                let mut interval_string = message.clone().text.unwrap();
                let interval_vec: Vec<&str> = interval_string.split(":").collect();
                match interval_vec.len() {
                    3 => (),
                    2 => interval_string = String::from("00:") + &*message.clone().text.unwrap(),
                    1 => interval_string = String::from("00:00:") + &*message.clone().text.unwrap(),
                    _ => return Some(String::from("Interval changing error!"))
                }
                self.word_update_interval = interval_string.clone();
                self.change_field(&collection, "word_update_interval", interval_string.clone()).await;
                self.change_next_word_update_datetime(database.clone()).await;
                self.change_state(Idle, database.clone()).await;
                Some(String::from("Interval was changed!"))
            },
            UserState::LanguageChanging => None,
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
        let collection:Collection<TelegramUser> = database.collection(COLLECTION_NAME);
        self.prev_word_update_datetime = Clock::get_current_datetime().format("%Y-%m-%d %H:%M:%S").to_string();
        self.next_word_update_datetime = Clock::add_interval_time_to_time(
            Clock::get_current_datetime(),
            self.word_update_interval.parse().unwrap()
        ).format("%Y-%m-%d %H:%M:%S").to_string();
        self.change_field(&collection, "prev_word_update_datetime", self.prev_word_update_datetime.clone()).await;
        self.change_field(&collection, "next_word_update_datetime", self.next_word_update_datetime.clone()).await;
    }

    async fn change_field(&mut self, collection: &Collection<TelegramUser>, field: &str, value: String) {
        let update_modifications = UpdateModifications::from(
            doc!{"$set": {field: value}}
        );
        collection.find_one_and_update(
            doc! {"user_id": self.user_id.clone()},
            update_modifications,
            None
        ).await.expect("Error find and update");
    }
}