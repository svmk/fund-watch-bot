#[derive(Debug, derive_more::Display)]
pub struct RelativeUrl(String);

impl RelativeUrl {
    pub fn new(value: String) -> RelativeUrl {
        return RelativeUrl(value);
    }

    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}
