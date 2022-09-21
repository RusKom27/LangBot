use std::env;
use frankenstein::{AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, Update, UpdateContent};
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver};
use crate::db_handle::models::TelegramUser;


#[feature(unstable_trait)]
#[async_trait]
pub trait TelegramApiHandle {
    fn connect() -> AsyncApi;
    async fn get_message_simple_params(&self, user_id: String, message: &str) -> SendMessageParams;
    async fn listen(&self) -> Receiver<Message>;
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

    async fn listen(&self) -> Receiver<Message> {
        let update_params_builder = GetUpdatesParams::builder();
        let mut update_params = update_params_builder.clone().build();
        let (sender, mut receiver) = mpsc::channel(32);
        let api = self.clone();
        tokio::spawn(async move {
            loop {
                let result = api.get_updates(&update_params).await;
                println!("result: {:?}", result);
                match result {
                    Ok(response) => {
                        for update in response.result {
                            if let UpdateContent::Message(message) = update.content {
                                sender.send(message.clone()).await.expect("Send error!");
                                // TelegramUser::add(
                                //     self.database.clone(),
                                //     message.from.unwrap().id
                                // ).await;
                                // let send_message_params = SendMessageParams::builder()
                                //     .chat_id(message.chat.id)
                                //     .text(format!("hello{}", message.chat.id))
                                //     .build();
                                // self.telegram_api.send_message(&send_message_params).await.expect("Fail to send the message!");

                                update_params = update_params_builder
                                    .clone()
                                    .offset(update.update_id + 1)
                                    .build();
                            }
                        }
                    }
                    Err(error) => {
                        println!("Failed to get updates: {:#?}", error);
                    }
                }


                // if self.clock.check_clock().await {
                //     match TelegramUser::get_with_closer_update(self.database.clone()).await {
                //         Some(mut user) => {
                //             user.change_next_word_update_datetime(self.database.clone()).await;
                //
                //             self.telegram_api.send_message(
                //                 &self.telegram_api.get_message_simple_params(
                //                     user.clone().user_id,
                //                     &user.next_word_update_datetime
                //                 ).await
                //             ).await.expect("Error send message");
                //
                //             self.clock.set_next_update(
                //                 NaiveDateTime::parse_from_str(
                //                     &TelegramUser::get_with_closer_update(self.database.clone()).await
                //                         .expect("Error get closer update!").next_word_update_datetime,
                //                     "%Y-%m-%d %H:%M:%S"
                //             ).expect("Parsing from str error!"));
                //         },
                //         None => continue
                //     }
                // }
            }
        }).await.expect("Start listening error!");
        receiver
    }
}