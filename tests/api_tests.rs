use ask::{AskError, ClaudeClient, Message};
use serde_json::json;

#[tokio::test]
async fn test_claude_client_creation() {
    let client = ClaudeClient::new("test-api-key".to_string());
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_claude_client_with_custom_timeout() {
    let client = ClaudeClient::with_timeout("test-api-key".to_string(), 60);
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_message_creation() {
    let user_message = Message::user("Hello, world!".to_string());
    assert_eq!(user_message.role, "user");
    assert_eq!(user_message.content, "Hello, world!");

    let assistant_message = Message::assistant("Hello there!".to_string());
    assert_eq!(assistant_message.role, "assistant");
    assert_eq!(assistant_message.content, "Hello there!");
}

#[test]
fn test_chat_request_serialization() {
    use ask::client::models::ChatRequest;

    let messages = vec![
        Message::user("Hello".to_string()),
        Message::assistant("Hi there!".to_string()),
    ];

    let request = ChatRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 1000,
        messages,
        temperature: Some(0.7),
        stream: Some(true),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("claude-3-5-sonnet-20241022"));
    assert!(json.contains("Hello"));
    assert!(json.contains("Hi there!"));
}

#[test]
fn test_error_response_deserialization() {
    use ask::client::models::ErrorResponse;

    let json = json!({
        "type": "error",
        "error": {
            "type": "authentication_error",
            "message": "Invalid API key"
        }
    });

    let error_response: ErrorResponse = serde_json::from_value(json).unwrap();
    assert_eq!(error_response.r#type, "error");
    assert_eq!(error_response.error.r#type, "authentication_error");
    assert_eq!(error_response.error.message, "Invalid API key");
}

#[test]
fn test_stream_event_parsing() {
    use ask::client::models::StreamEvent;

    let json = json!({
        "type": "content_block_delta",
        "index": 0,
        "delta": {
            "type": "text_delta",
            "text": "Hello"
        }
    });

    let event: StreamEvent = serde_json::from_value(json).unwrap();
    assert_eq!(event.r#type, "content_block_delta");
    assert!(event.delta.is_some());
    assert_eq!(event.delta.unwrap().text, "Hello");
}

#[test]
fn test_conversation_management() {
    use ask::client::models::Conversation;

    let mut conversation = Conversation::new("Test Conversation".to_string());
    assert_eq!(conversation.title, "Test Conversation");
    assert!(conversation.messages.is_empty());
    assert!(!conversation.id.is_empty());

    let user_message = Message::user("Hello".to_string());
    conversation.add_message(user_message);
    assert_eq!(conversation.messages.len(), 1);
    assert_eq!(conversation.messages[0].content, "Hello");

    let assistant_message = Message::assistant("Hi there!".to_string());
    conversation.add_message(assistant_message);
    assert_eq!(conversation.messages.len(), 2);
}

#[test]
fn test_ask_error_from_conversions() {
    // reqwest::Error から AskError への変換をテスト
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let ask_error: AskError = io_error.into();
    assert!(matches!(ask_error, AskError::IoError(_)));

    // serde_json::Error から AskError への変換をテスト
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let ask_error: AskError = json_error.into();
    assert!(matches!(ask_error, AskError::SerializationError(_)));
}
