use crate::fetching::model::url::Url;
use crate::fetching::model::mime_type::MimeType;
#[derive(Debug)]
pub struct Request {
    url: Url,
    check_status_code: bool,
    expected_mimes: Vec<MimeType>,
}

impl Request {
    pub fn get(url: Url) -> Request {
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

    pub fn get_url(&self) -> &Url {
        return &self.url;
    }

    pub fn get_check_status_code(&self) -> bool {
        return self.check_status_code;
    }

    pub fn get_expected_mimes(&self) -> &Vec<MimeType> {
        return &self.expected_mimes;;
    }
}
