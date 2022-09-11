mod db_handle;
mod clock_handle;
mod env_vars;


use chrono::NaiveTime;
use std::env;
use frankenstein::{ AsyncTelegramApi, AsyncApi, UpdateContent, GetUpdatesParams, SendMessageParams };

use crate::db_handle::{ DatabaseHandle };
use crate::clock_handle::{ Clock };
use crate::env_vars::get_vars;


#[tokio::main]
async fn main() {
    get_vars();
    let telegram_api = AsyncApi::new(&env::var("TELEGRAM_TOKEN").expect("Token not found"));
    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    let database_handle = DatabaseHandle::new("Test2", "users").await;
    let mut clock_handle = Clock::new(NaiveTime::from_hms(0,0,30));

    loop {
        let result = telegram_api.get_updates(&update_params).await;

        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        database_handle.add_telegram_user(
                            message.from.unwrap().id,
                            message.chat.id
                        ).await.expect("Adding user error");

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
                println!("Failed to get updates: {:?}", error);
            }
        }


        if clock_handle.check_clock().await {
            let mut users_cursor = database_handle.get_telegram_users().await;
            while users_cursor.advance().await.expect("Get users error") {
                let chat_id = users_cursor.current().get_str("chat_id").unwrap().parse::<i64>().unwrap();
                let send_message_params = SendMessageParams::builder()
                    .chat_id(chat_id)
                    .text("hello")
                    .build();
                telegram_api.send_message(&send_message_params).await.expect("Fail to send the message!");
            }

        }
    }
}

