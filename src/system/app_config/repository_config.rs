use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConfig {
    #[serde(rename="path")]
    path: PathBuf,
}

impl RepositoryConfig {
    pub fn get_path(&self) -> PathBuf {
        return self.path.clone();
    }
}