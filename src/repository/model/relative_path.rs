use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RelativePath(PathBuf);

impl RelativePath {
    pub fn from_string(value: String) -> RelativePath {
        let mut path = PathBuf::new();
        path.push(value);
        return RelativePath(path);
    }

    pub fn into_path_buf(self) -> PathBuf {
        return self.0;
    }
}
