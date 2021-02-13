use crate::fetching::service::http_client::request::Request;
use crate::fetching::model::url::Url;
use crate::fetching::model::mime_type::MimeType;

#[derive(Debug)]
pub struct FileDownloadRequest {
    pub request: Request,
}

impl FileDownloadRequest {
    pub fn new(url: Url, expected_mimes: Vec<MimeType>) -> FileDownloadRequest {
        let mut request = Request::get(url);
        request.expected_mimes = expected_mimes;
        return FileDownloadRequest {
            request,
        };
    }
}