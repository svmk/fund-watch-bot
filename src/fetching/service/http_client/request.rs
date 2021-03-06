use crate::prelude::*;
use crate::fetching::model::url::Url;
use crate::fetching::model::mime_type::MimeType;
use crate::fetching::service::http_client::request_method::RequestMethod;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Request {
    url: Url,
    request_method: RequestMethod,
    check_status_code: bool,
    expected_mimes: Vec<MimeType>,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
    retry_delay: Option<Duration>,
}

impl Request {
    pub fn get(url: Url) -> Request {
        return Request {
            url,
            request_method: RequestMethod::Get,
            check_status_code: true,
            expected_mimes: Vec::new(),
            headers: HeaderMap::new(),
            body: None,
            retry_delay: None,
        };
    }

    pub fn post(url: Url) -> Request {
        return Request {
            url,
            request_method: RequestMethod::Post,
            check_status_code: true,
            expected_mimes: Vec::new(),
            headers: HeaderMap::new(),
            body: None,
            retry_delay: None,
        };
    }

    pub fn with_mime_type(mut self, mime_type: MimeType) -> Self {
        self.expected_mimes.push(mime_type);
        return self;
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Result<Self, Failure> {
        if !self.get_method().has_body() {
            return Err(Failure::msg("Body not acceptable for this request"));
        }
        self.body = Some(body.into());
        return Ok(self);
    }

    pub fn with_header(mut self, key: &'static str, value: &str) -> Result<Self, Failure> {
        let value = HeaderValue::from_str(value)?;
        let _ = self.headers.insert(key, value);
        return Ok(self);
    }

    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = Some(delay);
        return self;
    }

    pub fn get_method(&self) -> &RequestMethod {
        return &self.request_method;
    }

    pub fn get_url(&self) -> &Url {
        return &self.url;
    }

    pub fn get_check_status_code(&self) -> bool {
        return self.check_status_code;
    }

    pub fn get_expected_mimes(&self) -> &Vec<MimeType> {
        return &self.expected_mimes;
    }

    pub fn get_headers(&self) -> &HeaderMap {
        return &self.headers;
    }

    pub fn take_body(&mut self) -> Option<Vec<u8>> {
        return self.body.take();
    }

    pub fn get_retry_delay(&self) -> Option<&Duration> {
        return self.retry_delay.as_ref();
    }
}
