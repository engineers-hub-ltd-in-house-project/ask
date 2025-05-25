use ask::config::OutputFormat;
use ask::{ConfigManager, Settings};
use std::env;

#[tokio::test]
async fn test_config_manager_operations() {
    let config_manager = ConfigManager::new();

    // デフォルト設定のロード
    let settings = config_manager.load_settings().unwrap();
    assert_eq!(settings.api.model, "claude-3-5-sonnet-20241022");
    assert_eq!(settings.api.max_tokens, 4096);
    assert!(settings.api.stream);
}

#[test]
fn test_api_config_default() {
    use ask::config::ApiConfig;

    let api_config = ApiConfig::default();
    assert_eq!(api_config.model, "claude-3-5-sonnet-20241022");
    assert_eq!(api_config.timeout, 30);
    assert_eq!(api_config.max_tokens, 4096);
    assert!(api_config.stream);
    assert_eq!(api_config.temperature, 0.7);
}

#[test]
fn test_output_config_default() {
    use ask::config::OutputConfig;

    let output_config = OutputConfig::default();
    assert!(matches!(output_config.format, OutputFormat::Plain));
    assert!(output_config.color);
    assert!(!output_config.verbose);
    assert!(!output_config.pager);
}

#[test]
fn test_conversation_config_default() {
    use ask::config::ConversationConfig;

    let conversation_config = ConversationConfig::default();
    assert!(conversation_config.save_history);
    assert_eq!(conversation_config.max_history_entries, 1000);
    assert!(conversation_config.auto_title);
}

#[test]
fn test_template_config_default() {
    use ask::config::TemplateConfig;

    let template_config = TemplateConfig::default();
    assert!(!template_config.default_template_dir.is_empty());
    assert!(template_config.auto_load);
}

#[test]
fn test_output_format_from_cli() {
    use ask::cli::OutputFormat as CliOutputFormat;
    use ask::config::OutputFormat as ConfigOutputFormat;

    let plain: ConfigOutputFormat = CliOutputFormat::Plain.into();
    assert!(matches!(plain, ConfigOutputFormat::Plain));

    let json: ConfigOutputFormat = CliOutputFormat::Json.into();
    assert!(matches!(json, ConfigOutputFormat::Json));

    let markdown: ConfigOutputFormat = CliOutputFormat::Markdown.into();
    assert!(matches!(markdown, ConfigOutputFormat::Markdown));
}

#[test]
fn test_settings_serialization_roundtrip() {
    let original_settings = Settings::default();

    // Serialize to JSON
    let json = serde_json::to_string(&original_settings).unwrap();

    // Deserialize back
    let deserialized_settings: Settings = serde_json::from_str(&json).unwrap();

    // Check that values match
    assert_eq!(original_settings.api.model, deserialized_settings.api.model);
    assert_eq!(
        original_settings.api.max_tokens,
        deserialized_settings.api.max_tokens
    );
    assert_eq!(
        original_settings.api.temperature,
        deserialized_settings.api.temperature
    );
    assert_eq!(
        original_settings.api.stream,
        deserialized_settings.api.stream
    );
}

#[test]
fn test_environment_variable_precedence() {
    let config_manager = ConfigManager::new();

    // Set both environment variables
    env::set_var("ANTHROPIC_API_KEY", "anthropic-key");
    env::set_var("ASK_API_KEY", "ask-key");

    // ANTHROPIC_API_KEY should take precedence
    let key = config_manager.get_api_key_from_env();
    assert_eq!(key, Some("anthropic-key".to_string()));

    // Remove ANTHROPIC_API_KEY, ASK_API_KEY should be used
    env::remove_var("ANTHROPIC_API_KEY");
    let key = config_manager.get_api_key_from_env();
    assert_eq!(key, Some("ask-key".to_string()));

    // Clean up
    env::remove_var("ASK_API_KEY");
    let key = config_manager.get_api_key_from_env();
    assert_eq!(key, None);
}

#[test]
fn test_config_file_path() {
    let config_manager = ConfigManager::new();
    let path = config_manager.get_config_path().unwrap();

    // Path should contain the app name
    assert!(path.to_string_lossy().contains("ask"));
}

#[test]
fn test_settings_clone() {
    let settings = Settings::default();
    let cloned_settings = settings.clone();

    assert_eq!(settings.api.model, cloned_settings.api.model);
    assert_eq!(settings.api.max_tokens, cloned_settings.api.max_tokens);
}

#[test]
fn test_output_format_display() {
    assert_eq!(OutputFormat::Plain.to_string(), "plain");
    assert_eq!(OutputFormat::Json.to_string(), "json");
    assert_eq!(OutputFormat::Markdown.to_string(), "markdown");
}

#[test]
fn test_cli_export_format_display() {
    use ask::cli::ExportFormat;

    assert_eq!(ExportFormat::Json.to_string(), "json");
    assert_eq!(ExportFormat::Markdown.to_string(), "markdown");
    assert_eq!(ExportFormat::Text.to_string(), "text");
}

#[test]
fn test_debug_formatting() {
    let settings = Settings::default();
    let debug_str = format!("{:?}", settings);

    // Debug formatting should include struct name and field values
    assert!(debug_str.contains("Settings"));
    assert!(debug_str.contains("ApiConfig"));
}
