use ask::{run_app, AskError};
use colored::*;
use std::process;

#[tokio::main]
async fn main() {
    // カラー出力の初期化
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    // アプリケーションの実行
    if let Err(e) = run_app().await {
        handle_error(e);
        process::exit(1);
    }
}

fn handle_error(error: AskError) {
    match error {
        AskError::AuthenticationFailed => {
            eprintln!("{} Authentication failed", "❌ Error:".red().bold());
            eprintln!(
                "Please set your API key using: {}",
                "ask config set-key YOUR_API_KEY".cyan()
            );
            eprintln!(
                "Or set the {} environment variable",
                "ANTHROPIC_API_KEY".yellow()
            );
        }

        AskError::NetworkError(e) => {
            eprintln!("{} {}", "❌ Network Error:".red().bold(), e);
            eprintln!("Please check your internet connection and try again.");
        }

        AskError::ConfigError(msg) => {
            eprintln!("{} {}", "❌ Configuration Error:".red().bold(), msg);
            eprintln!("Try running: {}", "ask config show".cyan());
        }

        AskError::KeyringError(_) => {
            eprintln!("{} Failed to access keyring", "❌ Error:".red().bold());
            eprintln!("Try setting your API key as an environment variable:");
            eprintln!("  export ANTHROPIC_API_KEY=your_api_key");
        }

        AskError::InvalidInput(msg) => {
            eprintln!("{} {}", "❌ Invalid Input:".red().bold(), msg);
            eprintln!("Run {} for help", "ask --help".cyan());
        }

        AskError::RateLimitExceeded => {
            eprintln!("{} Rate limit exceeded", "❌ Error:".red().bold());
            eprintln!("Please wait a moment and try again.");
        }

        _ => {
            eprintln!("{} {}", "❌ Error:".red().bold(), error);
        }
    }
}
