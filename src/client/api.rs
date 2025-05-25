use crate::client::models::{ChatRequest, ChatResponse, ErrorResponse, Message};
use crate::error::{AskError, Result};
use reqwest::{Client, Response};
use std::time::Duration;
use tokio_stream::Stream;

pub struct ClaudeClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        Ok(Self {
            client,
            api_key,
            base_url: "https://api.anthropic.com/v1".to_string(),
        })
    }

    pub fn with_timeout(api_key: String, timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;

        Ok(Self {
            client,
            api_key,
            base_url: "https://api.anthropic.com/v1".to_string(),
        })
    }

    /// 単一メッセージを送信する
    pub async fn send_message(
        &self,
        message: &str,
        model: &str,
        max_tokens: u32,
        temperature: Option<f32>,
    ) -> Result<String> {
        let messages = vec![Message::user(message.to_string())];
        let response = self
            .send_messages(messages, model, max_tokens, temperature, false)
            .await?;

        if let Some(content_block) = response.content.first() {
            Ok(content_block.text.clone())
        } else {
            Err(AskError::InvalidInput(
                "Empty response from API".to_string(),
            ))
        }
    }

    /// 複数メッセージ（会話履歴）を送信する
    pub async fn send_messages(
        &self,
        messages: Vec<Message>,
        model: &str,
        max_tokens: u32,
        temperature: Option<f32>,
        stream: bool,
    ) -> Result<ChatResponse> {
        let request = ChatRequest {
            model: model.to_string(),
            max_tokens,
            messages,
            temperature,
            stream: if stream { Some(true) } else { None },
        };

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// ストリーミング形式でメッセージを送信する
    pub async fn stream_message(
        &self,
        message: &str,
        model: &str,
        max_tokens: u32,
        temperature: Option<f32>,
    ) -> Result<impl Stream<Item = Result<String>>> {
        let messages = vec![Message::user(message.to_string())];
        self.stream_messages(messages, model, max_tokens, temperature)
            .await
    }

    /// ストリーミング形式で複数メッセージを送信する
    pub async fn stream_messages(
        &self,
        messages: Vec<Message>,
        model: &str,
        max_tokens: u32,
        temperature: Option<f32>,
    ) -> Result<impl Stream<Item = Result<String>>> {
        let request = ChatRequest {
            model: model.to_string(),
            max_tokens,
            messages,
            temperature,
            stream: Some(true),
        };

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error_response(response).await);
        }

        Ok(crate::client::streaming::create_stream(response))
    }

    async fn handle_response(&self, response: Response) -> Result<ChatResponse> {
        if response.status().is_success() {
            let chat_response: ChatResponse = response.json().await?;
            Ok(chat_response)
        } else {
            Err(self.handle_error_response(response).await)
        }
    }

    async fn handle_error_response(&self, response: Response) -> AskError {
        let status = response.status();

        match status.as_u16() {
            401 => AskError::AuthenticationFailed,
            429 => AskError::RateLimitExceeded,
            _ => {
                if let Ok(error_response) = response.json::<ErrorResponse>().await {
                    AskError::InvalidInput(format!(
                        "API Error ({}): {}",
                        status, error_response.error.message
                    ))
                } else {
                    AskError::InvalidInput(format!("HTTP Error: {}", status))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ClaudeClient::new("test-key".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_with_timeout() {
        let client = ClaudeClient::with_timeout("test-key".to_string(), 60);
        assert!(client.is_ok());
    }
}
