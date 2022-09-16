mod db_handle;
mod clock_handle;
mod env_vars;

use chrono::NaiveTime;
use std::env;
use frankenstein::{AsyncTelegramApi, AsyncApi, UpdateContent, GetUpdatesParams, SendMessageParams};

use crate::db_handle::{ DatabaseHandle };
use crate::db_handle::models::{ TelegramUser };
use crate::clock_handle::{ Clock };
use crate::env_vars::get_vars;


#[tokio::main]
async fn main() {
    get_vars();
    let telegram_api = AsyncApi::new(&env::var("TELEGRAM_TOKEN").expect("Token not found"));
    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    let database_handle = DatabaseHandle::new("LangBotDataBase").await;
    let mut clock_handle = Clock::new(NaiveTime::from_hms(0,0,30));

    loop {
        let result = telegram_api.get_updates(&update_params).await;

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        TelegramUser::add(
                            database_handle.database.clone(),
                            message.from.unwrap().id
                        ).await;
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(format!("hello{}", message.chat.id))
                            .build();
                        telegram_api.send_message(&send_message_params).await.expect("Fail to send the message!");

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


        if clock_handle.check_clock().await {
            match TelegramUser::get_with_closer_update(database_handle.database.clone()).await {
                Some(mut user) => {
                    send_message(
                        telegram_api.clone(), user.clone().user_id
                    ).await;
                    user.change_next_word_update_datetime(database_handle.database.clone()).await;

                    clock_handle.set_interval(
                        TelegramUser::get_with_closer_update(
                            database_handle.database.clone()
                        ).await.expect("Error get closer update!").next_word_update_datetime.parse().unwrap()
                    );
                },
                None => continue
            }



        }
    }
}

async fn send_message(telegram_api: AsyncApi, user_id: String) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(user_id)
        .text("hello")
        .build();
    telegram_api.send_message(&send_message_params).await.expect("Fail to send the message!");
}

