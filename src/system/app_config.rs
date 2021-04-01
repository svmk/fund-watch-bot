use crate::fetching::service::http_client::HttpClientConfig;
use crate::yahoo_finance::service::yahoo_api::YahooApiConfig;
use crate::openfigi::service::openfigi_api::OpenFigiApiConfig;
use crate::telegram::service::bot_instance::BotInstanceConfig;
use crate::telegram::task::telegram_bot_task::TelegramBotTaskConfig;
use crate::sec_gov::service::edgar_api::EdgarApiConfig;
mod repository_config;
pub use self::repository_config::RepositoryConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(rename="http_client", default)]
    http_client: HttpClientConfig,
    #[serde(rename="yahoo_api", default)]
    yahoo_api: YahooApiConfig,
    #[serde(rename="openfigi_api", default)]
    openfigi_api: OpenFigiApiConfig,
    #[serde(rename="telegram_bot")]
    bot_instance: BotInstanceConfig,
    #[serde(rename="telegram_bot_importing", default)]
    telegram_bot_task: TelegramBotTaskConfig,
    #[serde(rename="edgar_api", default)]
    edgar_api: EdgarApiConfig,
    #[serde(rename="repository")]
    repository: RepositoryConfig,
    #[serde(rename="sentry")]
    sentry: Option<String>,
}

impl AppConfig {
    pub fn get_http_client(&self) -> HttpClientConfig {
        return self.http_client.clone();
    }

    pub fn get_yahoo_api(&self) -> YahooApiConfig {
        return self.yahoo_api.clone();
    }

    pub fn get_openfigi_api(&self) -> OpenFigiApiConfig {
        return self.openfigi_api.clone();
    }

    pub fn get_bot_instance(&self) -> BotInstanceConfig {
        return self.bot_instance.clone();
    }

    pub fn get_edgar_api(&self) -> EdgarApiConfig {
        return self.edgar_api.clone();
    }

    pub fn get_repository(&self) -> RepositoryConfig {
        return self.repository.clone();
    }

    pub fn get_opt_sentry(&self) -> Option<&String> {
        return self.sentry.as_ref();
    }

    pub fn get_telegram_bot_task(&self) -> TelegramBotTaskConfig {
        return self.telegram_bot_task.clone();
    }
}

