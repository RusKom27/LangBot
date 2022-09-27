
use serde::{self, Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum UserState {
    Idle,
    IntervalChanging,
    LanguageChanging,
}

impl ToString for UserState{
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

