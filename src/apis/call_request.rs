use crate::models::general::llm::{ApiResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use tokio::join;

use reqwest::header::{HeaderMap, HeaderValue};

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not set");
    let org_key = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not set");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&org_key)
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion_request = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    let res: ApiResponse = client
        .post(url)
        .json(&chat_completion_request)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let messages = vec![Message {
            role: "user".to_string(),
            content: "Who won the 2020 World Cup?".to_string(),
        }];

        let res = call_gpt(messages).await;
        if let Ok(res) = res {
            println!("{}", res);
        } else {
            println!("{}", res.unwrap_err());
        }
    }
}
