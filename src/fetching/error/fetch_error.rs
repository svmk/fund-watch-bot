use crate::fetching::model::mime_type::MimeType;
use crate::prelude::*;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Download error: {0}")]
    Download(#[from] reqwest::Error),
    #[error("Wrong status code [{}] when downloading `{}`", format_status_code(.0), format_url(.0))]
    WrongStatusCode(reqwest::Response),
    #[error("Expected mime types `{}` when received `{provided_mime}` for URL `{}`", format_mime_types(.expected_mimes), format_url(.response))]
    ExpectedMimeType {expected_mimes: Vec<MimeType>, provided_mime: MimeType, response: reqwest::Response},
    #[error("Mime type not provided when expected `{}` for URL `{}`", format_mime_types(.expected_mimes), format_url(.response))]
    MimeTypeNotProvided {expected_mimes: Vec<MimeType>, response: reqwest::Response},
    #[error("{0}")]
    Custom(Failure),
}

impl FetchError {
    pub fn custom(error: impl Into<Failure>) -> FetchError {
        return FetchError::Custom(error.into());
    }
}

fn format_status_code(s: &reqwest::Response) -> String {
    format!("{}", s.status())
}

fn format_url(s: &reqwest::Response) -> String {
    format!("{}", s.url())
}

fn format_mime_types(items: &Vec<MimeType>) -> String {
    let items: Vec<String> = items
        .iter()
        .map(|item| {
            return format!("{}", item);
        })
        .collect();
    let items = items.join(",");
    return format!("[{}]", items);
}