mod clock_handle;
mod env_vars;
mod telegram_api_handle;
mod db_handle;
mod app;
mod user_command_handle;
mod state_handle;

use crate::app::App;

#[tokio::main]
async fn main() {
    App::new("LangBotDataBase").await
        .start().await;
}

