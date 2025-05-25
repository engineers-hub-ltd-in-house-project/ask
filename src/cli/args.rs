use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "ask")]
#[command(about = "A high-performance CLI for Claude AI")]
#[command(version)]
#[command(long_about = "
Ask is a fast and efficient command-line interface for interacting with Claude AI.
You can ask questions, have conversations, and manage your AI interactions.

Examples:
  ask \"What is Rust?\"
  ask -f input.txt
  ask -i
  echo \"Hello\" | ask
")]
pub struct Cli {
    /// Message to send to Claude
    pub message: Option<String>,

    /// Input file to read message from
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,

    /// Interactive mode - start a conversation
    #[arg(short, long)]
    pub interactive: bool,

    /// Output format
    #[arg(short, long, value_enum)]
    pub output: Option<OutputFormat>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Disable streaming (get full response at once)
    #[arg(long)]
    pub no_stream: bool,

    /// Model to use (overrides config)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Maximum tokens for the response
    #[arg(long)]
    pub max_tokens: Option<u32>,

    /// Temperature for randomness (0.0-1.0)
    #[arg(long)]
    pub temperature: Option<f32>,

    /// Conversation ID to continue
    #[arg(short = 'c', long)]
    pub conversation: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Conversation history management
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },
    /// Template management
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set API key
    SetKey {
        /// The API key to store
        key: String,
    },
    /// Show current configuration
    Show,
    /// Set default model
    SetModel {
        /// Model name (e.g., claude-3-5-sonnet-20241022)
        model: String,
    },
    /// Set default max tokens
    SetMaxTokens {
        /// Maximum number of tokens
        tokens: u32,
    },
    /// Set default temperature
    SetTemperature {
        /// Temperature value (0.0-1.0)
        temp: f32,
    },
    /// Set output format
    SetFormat {
        /// Output format
        #[arg(value_enum)]
        format: OutputFormat,
    },
    /// Reset configuration to defaults
    Reset,
    /// Show configuration file path
    Path,
}

#[derive(Subcommand)]
pub enum HistoryAction {
    /// List conversation history
    List {
        /// Limit number of conversations to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Show specific conversation
    Show {
        /// Conversation ID
        id: String,
    },
    /// Delete conversation
    Delete {
        /// Conversation ID
        id: String,
    },
    /// Clear all conversation history
    Clear {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
    /// Export conversation to file
    Export {
        /// Conversation ID
        id: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Export format
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,
    },
}

#[derive(Subcommand)]
pub enum TemplateAction {
    /// List available templates
    List,
    /// Show template content
    Show {
        /// Template name
        name: String,
    },
    /// Create new template
    Create {
        /// Template name
        name: String,
        /// Template content
        content: String,
    },
    /// Edit existing template
    Edit {
        /// Template name
        name: String,
    },
    /// Delete template
    Delete {
        /// Template name
        name: String,
    },
    /// Use template with variables
    Use {
        /// Template name
        name: String,
        /// Variables in key=value format
        #[arg(short, long)]
        var: Vec<String>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Plain text output
    Plain,
    /// JSON formatted output
    Json,
    /// Markdown formatted output
    Markdown,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
    /// Plain text format
    Text,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Plain => write!(f, "plain"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Markdown => write!(f, "markdown"),
        }
    }
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::Json => write!(f, "json"),
            ExportFormat::Markdown => write!(f, "markdown"),
            ExportFormat::Text => write!(f, "text"),
        }
    }
}
