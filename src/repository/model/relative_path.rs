use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RelativePath(PathBuf);

impl RelativePath {
    pub fn from_string(value: String) -> RelativePath {
        let mut path = PathBuf::new();
        path.push(value);
        return RelativePath(path);
    }
}
