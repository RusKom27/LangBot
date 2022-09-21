use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter};
use tokio::sync::mpsc::{Receiver};
use chrono::NaiveDateTime;
use frankenstein::{AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, Update, UpdateContent};
use mongodb::Database;
use crate::app::AppReceiver::{ClockReceiver, TelegramReceiver};

use crate::env_vars::get_vars;
use crate::db_handle::{DatabaseHandle};
use crate::telegram_api_handle::TelegramApiHandle;
use crate::db_handle::models::TelegramUser;
use crate::clock_handle::Clock;


pub enum AppReceiver {
    TelegramReceiver(Receiver<Message>),
    ClockReceiver(Receiver<bool>),
}

pub struct App {
    receivers: Vec<AppReceiver>
}

impl App {
    pub async fn new(database_name: &str) -> Self {
        get_vars();
        let mut receivers = Vec::<AppReceiver>::new();
        receivers.push(TelegramReceiver(AsyncApi::connect().listen().await));
        receivers.push(ClockReceiver(Clock::new().start().await));
        Self {
            receivers: Vec::new(),
        }
    }

    pub async fn process(&self) {
        loop {
            for receiver in &self.receivers {
                let app_receiver = receiver;
                match app_receiver {
                    TelegramReceiver(mut message_receiver) => {
                        while let Ok(results) = message_receiver.borrow_mut().try_recv() {
                            println!("{}", results.text.unwrap());
                        }
                    },
                    ClockReceiver(tick_receiver) => continue,
                    _ => continue,
                }
            }
        }
    }

    async fn check_telegram_receiver(mut receiver:Receiver<Message>) {
        while let Ok(result) = receiver.try_recv() {

        }
    }

    async fn check_clock_receiver(mut receiver:Receiver<bool>) -> bool {
        while let Ok(result) = receiver.try_recv() {
            return result;
        }
        false
    }
}