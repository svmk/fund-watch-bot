use crate::repository::file_storage::storage_instance::StorageInstance;
use crate::repository::model::relative_path::RelativePath;
use crate::repository::model::abs_file::AbsFile;
use crate::sec_gov::model::edgar_file::EdgarFile;
use crate::sec_gov::model::relative_url::RelativeUrl;
use crate::prelude::*;
use typed_di::service::Service;


#[derive(new, Debug)]
pub struct EdgarCache {
    repository: Service<StorageInstance<EdgarFile>>,
}

impl EdgarCache {
    pub async fn find(&self, url: &RelativeUrl) -> Result<Option<EdgarFile>, Failure> {
        let path = Self::get_relative_path(url);
        let is_exists = self.repository.exists(path.clone()).await?;
        if !is_exists {
            return Ok(None);
        }
        let file = self.repository.read(path).await?;
        return Ok(Some(file));
    }

    pub async fn get(&self, url: &RelativeUrl) -> Result<EdgarFile, Failure> {
        let path = Self::get_relative_path(url);
        let file = self.repository.read(path).await?;
        return Ok(file);
    }

    pub async fn replace(&self, url: &RelativeUrl, file: &dyn AbsFile) -> Result<(), Failure> {
        let path = Self::get_relative_path(url);
        self.repository.replace(path, file).await?;
        return Ok(());
    }

    fn get_relative_path(url: &RelativeUrl) -> RelativePath {
        let url = format!("{}", url);
        return RelativePath::from_string(url);
    }
}