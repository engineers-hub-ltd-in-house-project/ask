use crate::cli::args::{ConfigAction, HistoryAction, OutputFormat, TemplateAction};
use crate::config::{ConfigManager, Settings};
use crate::error::{AskError, Result};
use colored::*;
use std::io::{self, Write};

pub struct CommandHandler {
    config_manager: ConfigManager,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            config_manager: ConfigManager::new(),
        }
    }

    pub async fn handle_config_command(&self, action: ConfigAction) -> Result<()> {
        match action {
            ConfigAction::SetKey { key } => {
                self.config_manager.store_api_key(&key)?;
                println!("{}", "✅ API key stored successfully".green());
            }

            ConfigAction::Show => {
                let settings = self.config_manager.load_settings()?;

                println!("{}", "📋 Current Configuration".cyan().bold());
                println!("{}:", "API".yellow().bold());

                match self.config_manager.get_api_key_with_fallback() {
                    Ok(_) => println!("  API Key: {}", "✅ Set".green()),
                    Err(_) => println!("  API Key: {}", "❌ Not set".red()),
                }

                println!("  Model: {}", settings.api.model);
                println!("  Max Tokens: {}", settings.api.max_tokens);
                println!("  Temperature: {}", settings.api.temperature);
                println!(
                    "  Streaming: {}",
                    if settings.api.stream {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );

                println!("\n{}:", "Output".yellow().bold());
                println!("  Format: {}", settings.output.format);
                println!(
                    "  Color: {}",
                    if settings.output.color {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
                println!(
                    "  Verbose: {}",
                    if settings.output.verbose {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );

                println!("\n{}:", "Conversation".yellow().bold());
                println!(
                    "  Save History: {}",
                    if settings.conversation.save_history {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
                println!(
                    "  Max History: {}",
                    settings.conversation.max_history_entries
                );
                println!(
                    "  Auto Title: {}",
                    if settings.conversation.auto_title {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
            }

            ConfigAction::SetModel { model } => {
                let mut settings = self.config_manager.load_settings()?;
                settings.api.model = model.clone();
                self.config_manager.save_settings(&settings)?;
                println!("{} {}", "✅ Default model set to:".green(), model.cyan());
            }

            ConfigAction::SetMaxTokens { tokens } => {
                let mut settings = self.config_manager.load_settings()?;
                settings.api.max_tokens = tokens;
                self.config_manager.save_settings(&settings)?;
                println!(
                    "{} {}",
                    "✅ Max tokens set to:".green(),
                    tokens.to_string().cyan()
                );
            }

            ConfigAction::SetTemperature { temp } => {
                if !(0.0..=1.0).contains(&temp) {
                    return Err(AskError::InvalidInput(
                        "Temperature must be between 0.0 and 1.0".to_string(),
                    ));
                }
                let mut settings = self.config_manager.load_settings()?;
                settings.api.temperature = temp;
                self.config_manager.save_settings(&settings)?;
                println!(
                    "{} {}",
                    "✅ Temperature set to:".green(),
                    temp.to_string().cyan()
                );
            }

            ConfigAction::SetFormat { format } => {
                let mut settings = self.config_manager.load_settings()?;
                settings.output.format = format.clone().into();
                self.config_manager.save_settings(&settings)?;
                println!(
                    "{} {}",
                    "✅ Output format set to:".green(),
                    format.to_string().cyan()
                );
            }

            ConfigAction::Reset => {
                print!("❓ Are you sure you want to reset all configuration to defaults? [y/N]: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "y" {
                    let default_settings = Settings::default();
                    self.config_manager.save_settings(&default_settings)?;
                    println!("{}", "✅ Configuration reset to defaults".green());
                } else {
                    println!("{}", "❌ Reset cancelled".yellow());
                }
            }

            ConfigAction::Path => {
                let path = self.config_manager.get_config_path()?;
                println!("{} {}", "📁 Config file path:".cyan(), path.display());
            }
        }

        Ok(())
    }

    pub async fn handle_history_command(&self, action: HistoryAction) -> Result<()> {
        match action {
            HistoryAction::List { limit: _limit } => {
                println!("{}", "📚 Conversation History".cyan().bold());
                println!("(History management will be implemented with database)");
                // TODO: 実際のデータベース実装
            }

            HistoryAction::Show { id } => {
                println!("{} {}", "📖 Showing conversation:".cyan(), id.yellow());
                println!("(History viewing will be implemented with database)");
                // TODO: 実際のデータベース実装
            }

            HistoryAction::Delete { id } => {
                println!("{} {}", "🗑️  Deleting conversation:".yellow(), id.yellow());
                println!("(History deletion will be implemented with database)");
                // TODO: 実際のデータベース実装
            }

            HistoryAction::Clear { yes } => {
                if !yes {
                    print!("❓ Are you sure you want to clear all conversation history? [y/N]: ");
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if input.trim().to_lowercase() != "y" {
                        println!("{}", "❌ Clear cancelled".yellow());
                        return Ok(());
                    }
                }

                println!("{}", "🗑️ Clearing all conversation history...".yellow());
                println!("(History clearing will be implemented with database)");
                // TODO: 実際のデータベース実装
            }

            HistoryAction::Export { id, output, format } => {
                let output_file =
                    output.unwrap_or_else(|| format!("conversation_{}.{}", id, format));
                println!(
                    "{} {} {} {}",
                    "📤 Exporting conversation".cyan(),
                    id.yellow(),
                    "to".cyan(),
                    output_file.green()
                );
                println!("(History export will be implemented with database)");
                // TODO: 実際のデータベース実装
            }
        }

        Ok(())
    }

    pub async fn handle_template_command(&self, action: TemplateAction) -> Result<()> {
        match action {
            TemplateAction::List => {
                println!("{}", "📝 Available Templates".cyan().bold());
                println!("(Template management will be implemented)");
                // TODO: テンプレート実装
            }

            TemplateAction::Show { name } => {
                println!("{} {}", "📋 Template:".cyan(), name.yellow());
                println!("(Template viewing will be implemented)");
                // TODO: テンプレート実装
            }

            TemplateAction::Create { name, content } => {
                println!("{} {}", "✨ Creating template:".green(), name.yellow());
                println!("Content: {}", content);
                println!("(Template creation will be implemented)");
                // TODO: テンプレート実装
            }

            TemplateAction::Edit { name } => {
                println!("{} {}", "✏️  Editing template:".yellow(), name.yellow());
                println!("(Template editing will be implemented)");
                // TODO: テンプレート実装
            }

            TemplateAction::Delete { name } => {
                println!("{} {}", "🗑️  Deleting template:".red(), name.yellow());
                println!("(Template deletion will be implemented)");
                // TODO: テンプレート実装
            }

            TemplateAction::Use { name, var } => {
                println!("{} {}", "🚀 Using template:".green(), name.yellow());
                for v in var {
                    println!("  Variable: {}", v.cyan());
                }
                println!("(Template usage will be implemented)");
                // TODO: テンプレート実装
            }
        }

        Ok(())
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

// OutputFormat の変換実装
impl From<OutputFormat> for crate::config::OutputFormat {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Plain => crate::config::OutputFormat::Plain,
            OutputFormat::Json => crate::config::OutputFormat::Json,
            OutputFormat::Markdown => crate::config::OutputFormat::Markdown,
        }
    }
}
