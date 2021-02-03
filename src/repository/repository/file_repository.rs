use crate::prelude::*;
use crate::repository::model::identity::Identity;
use crate::repository::model::entity::Entity;
use crate::repository::path_resolver::path_resolver::PathResolver;
use crate::serializer::service::serializer_instance::SerializerInstance;
use crate::serializer::serializer::Serializer;
use std::marker::PhantomData;
use  async_std::path::Path as AsyncPath;
use async_std::fs::File;
use async_std::io::prelude::*;

pub struct FileRepository<I, T>
{
    _identity: PhantomData<I>,
    _entity: PhantomData<T>,
    path_resolver: Box<dyn PathResolver>,
    serializer_instance: SerializerInstance,
}

impl <I, T> FileRepository<I, T>
    where 
        I: Identity,
        T: Entity<I>,
{
    pub fn new(
        path_resolver: Box<dyn PathResolver>,
        serializer_instance: SerializerInstance,
    ) -> FileRepository<I, T> {
        return FileRepository {
            _identity: PhantomData{},
            _entity: PhantomData {},
            path_resolver,
            serializer_instance,
        };
    }

    pub async fn get(&self, id: &I) -> Result<T, Failure> {
        let path = self.path_resolver.resolve_path(id)?;
        let mut file = File::open(&path).await?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).await?;
        let model: T = self.serializer_instance.from_slice(&data)?;
        return Ok(model);
    }

    pub async fn find(&self, id: &I) -> Result<Option<T>, Failure> {
        let path = self.path_resolver.resolve_path(id)?;
        {
            let async_path = AsyncPath::new(&path);
            if !async_path.exists().await {
                return Ok(None);
            }
        }
        let mut file = File::open(&path).await?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).await?;
        let model: T = self.serializer_instance.from_slice(&data)?;
        return Ok(Some(model));
    }

    pub async fn store(&self, model: &T) -> Result<(), Failure> {
        let id = model.get_entity_id();
        let path = self.path_resolver.resolve_path(id)?;
        let mut file = File::open(&path).await?;
        let data = self.serializer_instance.to_vec(&model)?;
        file.write_all(&data).await?;
        return Ok(());
    }
}