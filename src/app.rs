
use chrono::NaiveDateTime;
use frankenstein::{AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, UpdateContent};
use mongodb::Database;

use crate::env_vars::get_vars;
use crate::clock_handle::Clock;
use crate::db_handle::DatabaseHandle;
use crate::db_handle::models::TelegramUser;
use crate::telegram_api_handle::TelegramApiHandle;
use crate::user_command_handle::UserCommand;


#[derive(Clone)]
pub struct App {
    telegram_api: AsyncApi,
    clock_api: Clock,
    database_api: Database,
}

impl App {
    pub async fn new(database_name: &str) -> Self {
        get_vars();
        let app = Self {
            telegram_api: AsyncApi::connect(),
            clock_api: Clock::new(),
            database_api: Database::connect(database_name).await,
        };
        app
    }

    pub async fn start(&mut self) {
        let update_params_builder = GetUpdatesParams::builder();
        let mut update_params = update_params_builder.clone().build();
        loop {
            let result = self.telegram_api.get_updates(&update_params).await;
            println!("result: {:?}", result);
            match result {
                Ok(response) => {
                    for update in response.result {
                        if let UpdateContent::Message(message) = update.content {
                            match TelegramUser::new(self.database_api.clone(), message.clone().from.unwrap().id.to_string()).await {
                                Some(mut user) =>
                                    match message.clone().parse_command() {
                                        Some(state) =>
                                            user.change_state(state, self.database_api.clone()).await,
                                        None =>
                                            user.change_params(&message, self.database_api.clone()).await,
                                    },
                                None => (),

                            }


                            let send_message_params = SendMessageParams::builder()
                                .chat_id(message.clone().chat.id)
                                .text(format!("{:?}", message.clone().parse_command()))
                                .build();
                            self.telegram_api.send_message(&send_message_params).await.expect("Fail to send the message!");

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


            if self.clock_api.check_clock().await {
                match TelegramUser::get_with_closer_update(self.database_api.clone()).await {
                    Some(mut user) => {
                        user.change_next_word_update_datetime(self.database_api.clone()).await;

                        self.telegram_api.send_message(
                            &self.telegram_api.get_message_simple_params(
                                user.clone().user_id,
                                &user.next_word_update_datetime
                            ).await
                        ).await.expect("Error send message");

                        self.clock_api.set_next_update(
                            NaiveDateTime::parse_from_str(
                                &TelegramUser::get_with_closer_update(self.database_api.clone()).await
                                    .expect("Error get closer update!").next_word_update_datetime,
                                "%Y-%m-%d %H:%M:%S"
                        ).expect("Parsing from str error!"));
                    },
                    None => continue
                }
            }
        }
    }
}