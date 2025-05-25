use crate::config::Settings;
use crate::error::{AskError, Result};
use keyring::Entry;
use std::path::PathBuf;

pub struct ConfigManager {
    app_name: &'static str,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self { app_name: "ask" }
    }

    /// 設定ファイルを読み込む
    pub fn load_settings(&self) -> Result<Settings> {
        match confy::load(self.app_name, None) {
            Ok(settings) => Ok(settings),
            Err(confy::ConfyError::BadTomlData(e)) => {
                eprintln!("Warning: Invalid config file format: {}", e);
                // デフォルト設定で継続
                Ok(Settings::default())
            }
            Err(e) => Err(AskError::ConfigFileError(e)),
        }
    }

    /// 設定ファイルを保存する
    pub fn save_settings(&self, settings: &Settings) -> Result<()> {
        confy::store(self.app_name, None, settings)?;
        Ok(())
    }

    /// 設定ファイルのパスを取得する
    pub fn get_config_path(&self) -> Result<PathBuf> {
        confy::get_configuration_file_path(self.app_name, None).map_err(AskError::ConfigFileError)
    }

    /// APIキーを安全に保存する
    pub fn store_api_key(&self, key: &str) -> Result<()> {
        let entry = Entry::new(self.app_name, "api-key")?;
        entry.set_password(key)?;
        Ok(())
    }

    /// APIキーを取得する
    pub fn get_api_key(&self) -> Result<String> {
        let entry = Entry::new(self.app_name, "api-key")?;
        let password = entry.get_password()?;
        Ok(password)
    }

    /// APIキーを削除する
    pub fn delete_api_key(&self) -> Result<()> {
        let entry = Entry::new(self.app_name, "api-key")?;
        entry.delete_password()?;
        Ok(())
    }

    /// 環境変数からAPIキーを取得する
    pub fn get_api_key_from_env(&self) -> Option<String> {
        std::env::var("ANTHROPIC_API_KEY")
            .or_else(|_| std::env::var("ASK_API_KEY"))
            .ok()
    }

    /// APIキーを取得する（優先順位: 環境変数 > キーリング）
    pub fn get_api_key_with_fallback(&self) -> Result<String> {
        if let Some(key) = self.get_api_key_from_env() {
            return Ok(key);
        }

        self.get_api_key()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
