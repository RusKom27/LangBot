use frankenstein::Message;
use crate::state_handle::UserState;
use crate::state_handle::UserState::{Idle, IntervalChanging, LanguageChanging};

pub trait UserCommand {
    fn parse_command(&self) -> Option<UserState>;
}

impl UserCommand for Message {
    fn parse_command(&self) -> Option<UserState> {
        match self.clone().text.expect("Get message text error!").as_str() {
            "/start" => Some(Idle),
            "/change_interval" => Some(IntervalChanging),
            "/change_language" => Some(LanguageChanging),
            _ => None,
        }
    }
}