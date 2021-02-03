use crate::fetching::model::url::Url;
use crate::fetching::model::mime_type::MimeType;
#[derive(Debug)]
pub struct Request {
    pub url: Url,
    pub check_status_code: bool,
    pub expected_mimes: Vec<MimeType>,
}

impl Request {
    pub fn new(url: Url) -> Request {
        return Request {
            url,
            check_status_code: true,
            expected_mimes: Vec::new(),
        };
    }

    pub fn with_mime_type(mut self, mime_type: MimeType) -> Self {
        self.expected_mimes = vec![mime_type];
        return self;
    }
}
