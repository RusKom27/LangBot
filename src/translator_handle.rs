use reqwest::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use std::env;

pub struct Translator {
    client: Client
}

impl Translator {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    async fn get_post_response(&self, text: &str, translate_to: &str) -> Option<Response> {
        let body_text = String::from("[{\"Text\":\"") + text + "\"}]";
        let post_query = self.client.post(format!(
            "https://microsoft-translator-text.p.rapidapi.com/translate?\
            to%5B0%5D={}&\
            api-version=3.0&\
            profanityAction=NoAction&\
            textType=plain&"
        , translate_to))
        .header(CONTENT_TYPE, "application/json")
        .header("X-RapidAPI-Key",&env::var("TRANSLATOR_TOKEN").expect("Token not found"))
        .header("X-RapidAPI-Host","microsoft-translator-text.p.rapidapi.com")
        .body(body_text);
        println!("{:#?}", post_query);
        post_query.send().await.ok()
    }

    pub async fn translate_text(&self, text: &str, translate_to: &str) -> Option<String> {
        let response_opt = self.get_post_response(&text, &translate_to).await;
        println!("{:#?}", response_opt);
        match response_opt {
            Some(response) => {
                let json_resp:Value = serde_json::from_str(
                    &response.text().await.expect("Get text from response error!")
                ).expect("From str error!");
                Some(json_resp[0]["translations"][0]["text"].to_string())
            },
            None => {
                None
            },

        }
    }
 }