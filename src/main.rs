mod clock_handle;
mod env_vars;
mod telegram_api_handle;
mod db_handle;
mod app;
mod state_handle;
mod translator_handle;
mod random_word_handle;

use crate::app::App;

#[tokio::main]
async fn main() {
    App::new("LangBotDataBase").await
        .start().await;
}

