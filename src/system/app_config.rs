use crate::fetching::service::http_client::HttpClientConfig;
use crate::yahoo_finance::service::yahoo_api::YahooApiConfig;
use crate::openfigi::service::openfigi_api::OpenFigiApiConfig;
use crate::telegram::service::bot_instance::BotInstanceConfig;
use crate::sec_gov::service::edgar_api::EdgarApiConfig;

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
    #[serde(rename="edgar_api", default)]
    edgar_api: EdgarApiConfig,
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
}