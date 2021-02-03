use crate::prelude::*;
use crate::fetching::model::mime_type::MimeType;
use std::str::FromStr;

#[derive(Debug)]
pub struct ContentType(mime::Mime);

impl ContentType {
    pub fn get_mime_type(&self) -> MimeType {
        let mut text = format!("{}", self.0.type_());
        if !self.0.subtype().as_str().is_empty() {
            text = format!("{}/{}", text, self.0.subtype());
        }
        if let Some(suffix) = self.0.suffix() {
            text = format!("{}+{}", text, suffix);
        }
        let mime_type = MimeType::from_str(&text).unwrap();
        return mime_type.into();
    }
}

impl FromStr for ContentType {
    type Err = Failure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mime_type = mime::Mime::from_str(s)?;
        return Ok(ContentType(mime_type));
    }
}