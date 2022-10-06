use std::borrow::{BorrowMut};
use std::env;
use frankenstein::{AsyncApi, AsyncTelegramApi, KeyboardButton, Message, ReplyKeyboardMarkup, ReplyMarkup, SendMessageParams};
use async_trait::async_trait;
use mongodb::Database;
use crate::db_handle::models::TelegramUser;
use crate::state_handle::UserState;
use crate::state_handle::UserState::{Idle, IntervalChanging, LanguageChanging};


#[async_trait]
pub trait TelegramApiHandle {
    fn connect() -> AsyncApi;
    async fn get_message_params(&self, user_id: String, message: &str, keys: Option<Vec<Vec<&str>>>) -> SendMessageParams;
    fn parse_message(message: Message) -> Option<UserState>;
    async fn message_handle(&self, message: Message, user:&mut TelegramUser, database_api: Database);
    async fn get_keyboard_from_keys(keys: Vec<Vec<&str>>) -> ReplyKeyboardMarkup;
}

#[async_trait]
impl TelegramApiHandle for AsyncApi {
    fn connect() -> AsyncApi {
        AsyncApi::new(&env::var("TELEGRAM_TOKEN").expect("Token not found"))
    }

    async fn get_message_params(&self, user_id: String, message: &str, keys: Option<Vec<Vec<&str>>>) -> SendMessageParams {
        match keys {
            Some(keys) =>
                SendMessageParams::builder()
                    .chat_id(user_id)
                    .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(Self::get_keyboard_from_keys(keys).await))
                    .text(message)
                    .build(),
            None =>
                SendMessageParams::builder()
                    .chat_id(user_id)
                    .text(message)
                    .build(),

        }
    }

    fn parse_message(message: Message) -> Option<UserState> {
        match message.clone().text.expect("Get message text error!").as_str() {
            "/start" => Some(Idle),
            "/change_interval" => Some(IntervalChanging),
            "/change_language" => Some(LanguageChanging),
            _ => None,
        }
    }

    async fn message_handle(&self, message: Message, user:&mut TelegramUser, database_api: Database) {
        let local_user = user.clone();
        let result_message = match Self::parse_message(message.clone()) {
            Some(state) =>
                user.borrow_mut().change_state(state, database_api.clone()).await,
            None =>
                user.borrow_mut().change_params(&message, database_api.clone()).await,
        };
        let user_id = local_user.clone().user_id;
        match result_message.0 {
            Some(message) => {
                self.send_message(
                    &self.get_message_params(
                        user_id,
                        &message,
                        result_message.1
                    ).await
                ).await.expect("Send message error!");
            },
            None => (),
        }
    }

    async fn get_keyboard_from_keys(keys: Vec<Vec<&str>>) -> ReplyKeyboardMarkup {
        let mut keyboard: Vec<Vec<KeyboardButton>> = Vec::new();
        for i in keys.clone() {
            let mut row: Vec<KeyboardButton> = Vec::new();
            for j in i {
                let name = format!("{}", j);
                let button = KeyboardButton::builder()
                    .text(name)
                    .build();
                row.push(button);
            }
            keyboard.push(row);
        }
        println!("{:#?}", keyboard);

        ReplyKeyboardMarkup::builder()
            .keyboard(keyboard)
            .build()
    }

}