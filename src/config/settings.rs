use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    pub api: ApiConfig,
    pub output: OutputConfig,
    pub conversation: ConversationConfig,
    pub template: TemplateConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub model: String,
    pub timeout: u64,
    pub max_tokens: u32,
    pub stream: bool,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub color: bool,
    pub verbose: bool,
    pub pager: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationConfig {
    pub save_history: bool,
    pub max_history_entries: usize,
    pub auto_title: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateConfig {
    pub default_template_dir: String,
    pub auto_load: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OutputFormat {
    Plain,
    Json,
    Markdown,
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

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            model: "claude-3-5-sonnet-20241022".to_string(),
            timeout: 30,
            max_tokens: 4096,
            stream: true,
            temperature: 0.7,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Plain,
            color: true,
            verbose: false,
            pager: false,
        }
    }
}

impl Default for ConversationConfig {
    fn default() -> Self {
        Self {
            save_history: true,
            max_history_entries: 1000,
            auto_title: true,
        }
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        let home_dir = directories::ProjectDirs::from("ltd", "engineers-hub", "ask")
            .map(|dirs| {
                dirs.config_dir()
                    .join("templates")
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or_else(|| ".ask/templates".to_string());

        Self {
            default_template_dir: home_dir,
            auto_load: true,
        }
    }
}
