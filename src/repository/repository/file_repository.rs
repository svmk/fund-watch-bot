use crate::prelude::*;
use crate::repository::model::identity::Identity;
use crate::repository::model::entity::Entity;
use crate::repository::path_resolver::path_resolver::PathResolver;
use crate::serializer::service::serializer_instance::SerializerInstance;
use crate::serializer::serializer::Serializer;
use std::marker::PhantomData;
use async_std::fs::File;
use async_std::io::ReadExt;

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
        let value: T = self.serializer_instance.from_slice(&data)?;
        return Ok(value);
    }    
}