#[derive(Debug)]
pub struct RelativePath(String);

impl RelativePath {
    pub fn new(value: String) -> RelativePath {
        return RelativePath(value);
    }

    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}
