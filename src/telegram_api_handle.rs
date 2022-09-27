use std::env;
use frankenstein::{AsyncApi, AsyncTelegramApi,  GetUpdatesParams, Message, SendMessageParams, UpdateContent};
use async_trait::async_trait;
use tokio::sync::{mpsc};
use tokio::sync::mpsc::{Receiver};


#[async_trait]
pub trait TelegramApiHandle {
    fn connect() -> AsyncApi;
    async fn get_message_simple_params(&self, user_id: String, message: &str) -> SendMessageParams;
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
}