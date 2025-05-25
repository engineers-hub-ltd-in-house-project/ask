//! # Ask CLI
//!
//! A high-performance command-line interface for Claude AI.
//!
//! ## Features
//!
//! - Fast and efficient Claude AI interactions
//! - Streaming responses
//! - Conversation history management
//! - Template system
//! - Multiple output formats
//! - Secure API key management
//!
//! ## Usage
//!
//! ```no_run
//! use ask::{ClaudeClient, ConfigManager};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Set API key via environment variable
//!     std::env::set_var("ANTHROPIC_API_KEY", "your-api-key");
//!     
//!     let config = ConfigManager::new();
//!     let api_key = config.get_api_key_with_fallback()?;
//!     let client = ClaudeClient::new(api_key)?;
//!     
//!     let response = client.send_message(
//!         "Hello, Claude!",
//!         "claude-3-5-sonnet-20241022",
//!         1000,
//!         Some(0.7)
//!     ).await?;
//!     
//!     println!("{}", response);
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod client;
pub mod config;
pub mod error;

// 基本的なアプリケーション機能を実装していく予定
// pub mod conversation;
// pub mod template;
// pub mod output;

// Public API exports
pub use cli::{Cli, CommandHandler};
pub use client::{ClaudeClient, Conversation, Message};
pub use config::{ConfigManager, Settings};
pub use error::{AskError, Result};

/// askライブラリのバージョン
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// askライブラリの名前
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// メインアプリケーションを実行するための便利関数
pub async fn run_app() -> Result<()> {
    use clap::Parser;

    let cli = Cli::parse();
    let command_handler = CommandHandler::new();
    let config_manager = ConfigManager::new();

    // サブコマンドの処理
    if let Some(command) = cli.command {
        match command {
            cli::Commands::Config { action } => {
                return command_handler.handle_config_command(action).await;
            }
            cli::Commands::History { action } => {
                return command_handler.handle_history_command(action).await;
            }
            cli::Commands::Template { action } => {
                return command_handler.handle_template_command(action).await;
            }
        }
    }

    // メイン機能: メッセージの処理
    let message = get_input_message(&cli).await?;

    if message.is_empty() {
        return Err(AskError::InvalidInput("No message provided".to_string()));
    }

    // 対話モードかどうかをチェック
    if cli.interactive {
        run_interactive_mode(&cli, &config_manager).await?;
    } else {
        run_single_message(&cli, &config_manager, &message).await?;
    }

    Ok(())
}

/// 入力メッセージを取得する
async fn get_input_message(cli: &Cli) -> Result<String> {
    use std::io::{self, Read};

    if let Some(ref message) = cli.message {
        return Ok(message.clone());
    }

    if let Some(ref file_path) = cli.file {
        let content = tokio::fs::read_to_string(file_path).await?;
        return Ok(content);
    }

    // パイプからの入力をチェック
    if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        return Ok(buffer);
    }

    Ok(String::new())
}

/// 単一メッセージモードを実行
async fn run_single_message(
    cli: &Cli,
    config_manager: &ConfigManager,
    message: &str,
) -> Result<()> {
    use colored::*;
    use std::io::{self, Write};
    use tokio_stream::StreamExt;

    let settings = config_manager.load_settings()?;
    let api_key = config_manager.get_api_key_with_fallback()?;

    let client = ClaudeClient::new(api_key)?;

    let model = cli.model.as_ref().unwrap_or(&settings.api.model);
    let max_tokens = cli.max_tokens.unwrap_or(settings.api.max_tokens);
    let temperature = cli.temperature.or(Some(settings.api.temperature));

    let use_streaming = !cli.no_stream && settings.api.stream;

    if use_streaming {
        // ストリーミングモード
        if cli.verbose {
            println!("{}", "🤔 Thinking...".yellow());
        }

        let stream = client
            .stream_message(message, model, max_tokens, temperature)
            .await?;
        let mut pinned_stream = Box::pin(stream);

        while let Some(chunk_result) = pinned_stream.next().await {
            match chunk_result {
                Ok(text) => {
                    print!("{}", text);
                    io::stdout().flush()?;
                }
                Err(e) => {
                    eprintln!("\n{} {}", "Error:".red(), e);
                    break;
                }
            }
        }
        println!(); // 改行
    } else {
        // 非ストリーミングモード
        if cli.verbose {
            println!("{}", "🤔 Processing request...".yellow());
        }

        let response = client
            .send_message(message, model, max_tokens, temperature)
            .await?;
        println!("{}", response);
    }

    Ok(())
}

/// 対話モードを実行
async fn run_interactive_mode(_cli: &Cli, _config_manager: &ConfigManager) -> Result<()> {
    use colored::*;

    println!("{}", "🎯 Interactive mode starting...".green().bold());
    println!(
        "{}",
        "Type 'exit' or 'quit' to end the conversation.".yellow()
    );
    println!("{}", "Type 'help' for available commands.".yellow());
    println!();

    // 対話モードの実装は後で行う
    println!(
        "{}",
        "(Interactive mode will be fully implemented)".yellow()
    );

    Ok(())
}

// 依存関係のre-export
pub use clap;
pub use serde;
pub use serde_json;
pub use tokio;
