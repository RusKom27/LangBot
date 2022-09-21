mod clock_handle;
mod env_vars;
mod telegram_api_handle;
mod db_handle;
mod app;

use crate::app::App;

#[tokio::main]
async fn main() {
    App::new("LangBotDataBase").await
        .process().await;
}

