use reqwest::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use std::env;

pub struct RandomWord {
    client: Client
}

impl RandomWord {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    async fn get_response(&self) -> Option<Response> {
        self.client.get("https://api.api-ninjas.com/v1/randomword")
            .header(CONTENT_TYPE, "application/json")
            .header("X-Api-Key", &env::var("RANDOM_WORD_TOKEN").expect("Token not found"))
            .send().await.ok()
    }

    pub async fn get_random_word(&self) -> Option<String> {
        let response_opt = self.get_response().await;
        match response_opt {
            Some(response) => {
                let json_resp:Value = serde_json::from_str(
                    &response.text().await.expect("Get text from response error!")
                ).expect("From str error!");
                Some(json_resp["word"].to_string().replace("\"", ""))
            },
            None => {
                None
            },

        }
    }
}