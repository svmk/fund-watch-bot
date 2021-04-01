use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TelegramBotTaskConfig {
    #[serde(rename="import_delay", default = "TelegramBotTaskConfig::default_import_delay")]
    import_delay: Duration,
}

impl TelegramBotTaskConfig {
    fn default_import_delay() -> Duration {
        return Duration::from_secs(6 * 60 * 6);
    }

    pub fn get_import_delay(&self) -> &Duration {
        return &self.import_delay;
    }
}

impl Default for TelegramBotTaskConfig {
    fn default() -> Self {
        return TelegramBotTaskConfig {
            import_delay: TelegramBotTaskConfig::default_import_delay(),
        }
    }
}
