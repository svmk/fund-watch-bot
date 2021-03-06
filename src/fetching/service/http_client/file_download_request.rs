use crate::fetching::service::http_client::request::Request;
use crate::fetching::model::url::Url;
use crate::fetching::model::mime_type::MimeType;
use std::time::Duration;

#[derive(Debug)]
pub struct FileDownloadRequest {
    pub request: Request,
}

impl FileDownloadRequest {
    pub fn new(url: Url, expected_mimes: Vec<MimeType>) -> FileDownloadRequest {
        let mut request = Request::get(url);
        for mime_type in expected_mimes {
            request = request.with_mime_type(mime_type);
        }
        return FileDownloadRequest {
            request,
        };
    }

    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.request = self.request.with_retry_delay(delay);
        return self;
    }
}