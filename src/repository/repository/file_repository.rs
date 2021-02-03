use crate::prelude::*;
use crate::repository::model::identity::Identity;
use crate::repository::model::entity::Entity;
use crate::repository::path_resolver::PathResolver;
use crate::repository::path_resolver_service::path_resolver_instance::PathResolverInstance;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::serializer::service::serializer_instance::SerializerInstance;
use crate::serializer::serializer::Serializer;
use std::marker::PhantomData;
use async_std::path::Path as AsyncPath;
use async_std::fs::File;
use async_std::io::prelude::*;
use async_std::fs::create_dir_all;

pub struct FileRepository<I, E>
{
    _identity: PhantomData<I>,
    _entity: PhantomData<E>,
    path_resolver: PathResolverInstance,
    serializer_instance: SerializerInstance,
}

impl <I, E> FileRepository<I, E>
    where 
        I: Identity,
        E: Entity<I>,
{
    pub fn new(
        path_resolver: PathResolverInstance,
        serializer_instance: SerializerInstance,
    ) -> RepositoryInstance<I, E> {
        let repository = FileRepository {
            _identity: PhantomData{},
            _entity: PhantomData {},
            path_resolver,
            serializer_instance,
        };
        return RepositoryInstance::FileRepository(repository);
    }

    pub async fn get(&self, id: &I) -> Result<E, Failure> {
        let path = self.path_resolver.resolve_path(id)?;
        let mut file = File::open(&path).await?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).await?;
        let model: E = self.serializer_instance.from_slice(&data)?;
        return Ok(model);
    }

    pub async fn find(&self, id: &I) -> Result<Option<E>, Failure> {
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
        let model: E = self.serializer_instance.from_slice(&data)?;
        return Ok(Some(model));
    }

    pub async fn store(&self, model: &E) -> Result<(), Failure> {
        let id = model.get_entity_id();
        let path = self.path_resolver.resolve_path(id)?;
        if let Some(dir) = path.parent() {
            create_dir_all(dir).await?;
        }
        let mut file = File::open(&path).await?;
        let data = self.serializer_instance.to_vec(&model)?;
        file.write_all(&data).await?;
        return Ok(());
    }
}