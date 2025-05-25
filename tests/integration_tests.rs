use ask::{AskError, ConfigManager, Settings};
use std::env;

#[tokio::test]
async fn test_config_manager_basic_operations() {
    let config_manager = ConfigManager::new();

    // デフォルト設定のロード
    let settings = config_manager.load_settings().unwrap();
    assert_eq!(settings.api.model, "claude-3-5-sonnet-20241022");
    assert_eq!(settings.api.max_tokens, 4096);
    assert_eq!(settings.api.temperature, 0.7);
    assert!(settings.api.stream);
}

#[test]
fn test_settings_default() {
    let settings = Settings::default();
    assert_eq!(settings.api.model, "claude-3-5-sonnet-20241022");
    assert_eq!(settings.api.max_tokens, 4096);
    assert_eq!(settings.api.temperature, 0.7);
    assert!(settings.api.stream);
    assert!(settings.output.color);
    assert!(!settings.output.verbose);
}

#[test]
fn test_environment_variable_api_key() {
    let config_manager = ConfigManager::new();

    // 既存のANTHROPIC_API_KEYを保存
    let original_anthropic_key = env::var("ANTHROPIC_API_KEY").ok();

    // 環境変数を設定
    env::set_var("ANTHROPIC_API_KEY", "test-key-123");

    // 環境変数からAPIキーを取得
    let key = config_manager.get_api_key_from_env();
    assert_eq!(key, Some("test-key-123".to_string()));

    // クリーンアップと復元
    if let Some(original_key) = original_anthropic_key {
        env::set_var("ANTHROPIC_API_KEY", original_key);
    } else {
        env::remove_var("ANTHROPIC_API_KEY");
    }
}

#[test]
fn test_ask_api_key_environment_variable() {
    let config_manager = ConfigManager::new();

    // 既存のANTHROPIC_API_KEYを保存して削除
    let original_anthropic_key = env::var("ANTHROPIC_API_KEY").ok();
    env::remove_var("ANTHROPIC_API_KEY");

    // ASK_API_KEY環境変数を設定
    env::set_var("ASK_API_KEY", "ask-key-456");

    // 環境変数からAPIキーを取得
    let key = config_manager.get_api_key_from_env();
    assert_eq!(key, Some("ask-key-456".to_string()));

    // クリーンアップ
    env::remove_var("ASK_API_KEY");

    // 元のANTHROPIC_API_KEYを復元
    if let Some(original_key) = original_anthropic_key {
        env::set_var("ANTHROPIC_API_KEY", original_key);
    }
}

#[test]
fn test_error_types() {
    // AskErrorの各種類をテスト
    let auth_error = AskError::AuthenticationFailed;
    assert_eq!(auth_error.to_string(), "API authentication failed");

    let invalid_input = AskError::InvalidInput("test error".to_string());
    assert_eq!(invalid_input.to_string(), "Invalid input: test error");

    let rate_limit = AskError::RateLimitExceeded;
    assert_eq!(rate_limit.to_string(), "Rate limit exceeded");
}

#[test]
fn test_config_path() {
    let config_manager = ConfigManager::new();
    let path = config_manager.get_config_path();
    assert!(path.is_ok());

    let path = path.unwrap();
    assert!(path.to_string_lossy().contains("ask"));
}

#[tokio::test]
async fn test_settings_serialization() {
    let settings = Settings::default();

    // 設定をJSONにシリアライズ
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("claude-3-5-sonnet-20241022"));

    // JSONから設定をデシリアライズ
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.api.model, settings.api.model);
    assert_eq!(deserialized.api.max_tokens, settings.api.max_tokens);
}

#[test]
fn test_output_format_display() {
    use ask::config::OutputFormat;

    assert_eq!(OutputFormat::Plain.to_string(), "plain");
    assert_eq!(OutputFormat::Json.to_string(), "json");
    assert_eq!(OutputFormat::Markdown.to_string(), "markdown");
}
