use crate::prelude::*;
use crate::repository::model::relative_path::RelativePath;
use crate::repository::model::file::File;
use crate::repository::file_storage::file_storage::FileStorage;
use crate::repository::model::abs_file::AbsFile;

#[derive(Debug)]
pub enum StorageInstance<F> {
    FileStorage(FileStorage<F>),
}

impl <F> StorageInstance<F>
    where 
        F: File,
{
    pub async fn read(&self, path: RelativePath) -> Result<F, Failure> {
        match self {
            StorageInstance::FileStorage(ref service) => {
                return service.read(path).await;
            },
        }
    }

    pub async fn write(&self, path: RelativePath) -> Result<F, Failure> {
        match self {
            StorageInstance::FileStorage(ref service) => {
                return service.write(path).await;
            },
        }
    }

    pub async fn exists(&self, path: RelativePath) -> Result<bool, Failure> {
        match self {
            StorageInstance::FileStorage(ref service) => {
                return service.exists(path).await;
            },
        }
    }

    pub async fn replace(&self, path: RelativePath, file: &dyn AbsFile) -> Result<(), Failure> {
        match self {
            StorageInstance::FileStorage(ref service) => {
                return service.replace(path, file).await;
            },
        }
    }
}