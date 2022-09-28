use std::env;
use frankenstein::{AsyncApi, AsyncTelegramApi, Message, SendMessageParams};
use async_trait::async_trait;
use mongodb::Database;
use crate::db_handle::models::TelegramUser;
use crate::state_handle::UserState;
use crate::state_handle::UserState::{Idle, IntervalChanging, LanguageChanging};


#[async_trait]
pub trait TelegramApiHandle {
    fn connect() -> AsyncApi;
    async fn get_message_simple_params(&self, user_id: String, message: &str) -> SendMessageParams;
    fn parse_message(message: Message) -> Option<UserState>;
    async fn message_handle(&self, message: Message, user:&mut TelegramUser, database_api: Database);
}

#[async_trait]
impl TelegramApiHandle for AsyncApi {
    fn connect() -> AsyncApi {
        AsyncApi::new(&env::var("TELEGRAM_TOKEN").expect("Token not found"))
    }

    async fn get_message_simple_params(&self, user_id: String, message: &str) -> SendMessageParams {
        SendMessageParams::builder()
            .chat_id(user_id)
            .text(message)
            .build()
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
        let result_message = match Self::parse_message(message.clone()) {
            Some(state) =>
                user.change_state(state, database_api.clone()).await,
            None =>
                user.change_params(&message, database_api.clone()).await,
        };
        match result_message {
            Some(message) => {
                self.send_message(
                    &self.get_message_simple_params(
                        user.user_id.clone(),
                        &message
                    ).await
                ).await.expect("Send message error!");
            },
            None => (),

        }
    }

}