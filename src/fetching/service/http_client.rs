use crate::prelude::*;
use crate::fetching::model::content_type::ContentType;
use crate::fetching::model::downloaded_file::DownloadedFile;
use crate::fetching::model::url::Url;
use crate::fetching::error::fetch_error::FetchError;
use futures::stream::StreamExt;
use async_std::fs::File;
use futures::AsyncWriteExt;
use async_std::task::sleep;
use std::str::FromStr;
mod request;
pub use self::request::Request;
mod file_download_request;
pub use self::file_download_request::FileDownloadRequest;
mod request_method;
pub use self::request_method::RequestMethod;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct HttpClientConfig {
    #[serde(default)]
    pub user_agent: String,
    #[serde(default)]
    pub proxy: Option<Url>,
}

impl HttpClientConfig {
    pub fn with_proxy_url(&self, url: Url) -> HttpClientConfig {
        let mut config = self.clone();
        config.proxy = Some(url);
        return config;
    }
}

#[derive(Debug)]
pub struct HttpClient {
    client: reqwest::Client,
    config: HttpClientConfig,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Result<HttpClient, Failure> {
        let mut builder = reqwest::ClientBuilder::new();
        if !config.user_agent.is_empty() {
            builder = builder.user_agent(config.user_agent.clone());
        }
        if let Some(ref proxy_url) = config.proxy {
            let proxy_url= format!("{}", proxy_url);
            builder = builder.proxy(reqwest::Proxy::all(&proxy_url)?);
        }
        let service = HttpClient {
            client: builder.build()?,
            config,
        };
        return Ok(service);
    }

    pub fn get_config(&self) -> &HttpClientConfig {
        return &self.config;
    }

    pub async fn send(&self, request: Request) -> Result<reqwest::Response, FetchError> {
        loop {
            let response_result = self
                .internal_send(request.clone()).await;
            let error = match response_result {
                Ok(response) => {
                    return Ok(response);
                },
                Err(error) => error,
            };
            let retry_delay = match error {
                FetchError::Download(..) => request.get_retry_delay(),
                FetchError::Custom(..) => request.get_retry_delay(),
                FetchError::WrongStatusCode(..) => None,
                FetchError::ExpectedMimeType{..} => None,
                FetchError::MimeTypeNotProvided {..} => None,
            };
            if let Some(retry_delay) = retry_delay {
                sleep(retry_delay.clone()).await;
            } else {
                return Err(error);
            }
        }
    }

    async fn internal_send(&self, mut request: Request) -> Result<reqwest::Response, FetchError> {
        let request_method = match request.get_method() {
            &RequestMethod::Get => {
                reqwest::Method::GET
            },
            &RequestMethod::Post => {
                reqwest::Method::POST
            },
        };
        let mut request_builder = self.client.request(request_method, request.get_url().clone());
        request_builder = request_builder.headers(request.get_headers().clone());
        if let Some(body) = request.take_body() {
            request_builder = request_builder.body(body);
        }
        let response = request_builder.send().await?;
        if request.get_check_status_code() {
            if response.status() != reqwest::StatusCode::OK {
                return Err(FetchError::WrongStatusCode(response));
            }
        }
        if !request.get_expected_mimes().is_empty() {
            let content_type = match response.headers().get(http::header::CONTENT_TYPE) {
                Some(content_type) => content_type,
                None => {
                    return Err(FetchError::MimeTypeNotProvided {expected_mimes: request.get_expected_mimes().clone(), response});
                },
            };
            let content_type = content_type.to_str().map_err(FetchError::custom)?;
            let content_type = ContentType::from_str(content_type).map_err(FetchError::custom)?;
            let provided_mime = content_type.get_mime_type();
            if request.get_expected_mimes().iter().find(|&expected_mime| {
                return expected_mime == &provided_mime;
            }).is_none() {
                return Err(FetchError::ExpectedMimeType {expected_mimes: request.get_expected_mimes().clone(), provided_mime, response});
            }
        }
        return Ok(response);
    }

    pub async fn fetch_file(&self, request: FileDownloadRequest) -> Result<DownloadedFile, FetchError> {
        let response = self.send(request.request).await?;
        let downloaded_file = DownloadedFile::new(response.url().clone()).map_err(FetchError::custom)?; 
        let mut file = File::create(downloaded_file.get_path()).await.map_err(FetchError::custom)?;
        let mut response_body = response.bytes_stream();
        while let Some(buffer) = response_body.next().await {
            let buffer = buffer?;
            file.write_all(&buffer).await.map_err(FetchError::custom)?;
        }
        file.sync_all().await.map_err(FetchError::custom)?;
        return Ok(downloaded_file);
    }
}