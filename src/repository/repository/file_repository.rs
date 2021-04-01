use crate::prelude::*;
use crate::repository::model::entity::Entity;
use crate::repository::model::identity::Identity;
use crate::repository::model::relative_path::RelativePath;
use crate::repository::model::query::Query;
use crate::repository::model::entity_stream::EntityStream;
use crate::repository::path_resolver::PathResolver;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::service::query_comparator::QueryComparator;
use crate::serializer::service::serializer_instance::SerializerInstance;
use crate::serializer::serializer::Serializer;
use crate::repository::utils::create_parent_dir::create_parent_dir;
use anyhow::Result;
use typed_di::service::service::Service;
use futures::stream::{StreamExt};
use std::marker::PhantomData;
use async_std::path::Path as AsyncPath;
use async_std::fs::File;
use async_std::io::prelude::*;
use async_walkdir::WalkDir;
use async_walkdir::Filtering;

pub struct FileRepository<E>
{
    _entity: PhantomData<E>,
    path_resolver: PathResolver,
    serializer_instance: SerializerInstance,
    query_comparator: Option<Service<QueryComparator>>,
}

impl <E> FileRepository<E>
    where 
        E: Entity,
        E::Id: Identity,
{
    pub fn new(
        path_resolver: PathResolver,
        serializer_instance: SerializerInstance,
    ) -> RepositoryInstance<E> {
        let repository = FileRepository {
            _entity: PhantomData {},
            path_resolver,
            serializer_instance,
            query_comparator: None,
        };
        return RepositoryInstance::FileRepository(repository);
    }

    pub fn with_query_comparator(mut self, query_comparator: Service<QueryComparator>) -> Self {
        self.query_comparator = Some(query_comparator);
        return self;
    }

    pub async fn get(&self, id: &E::Id) -> Result<E, Failure> {
        let path = RelativePath::from_string(id.to_string());
        let path = self.path_resolver.resolve_path(path)?;
        let mut file = File::open(&path).await
            .map_err(|error| {
                return crate::error!("Unable to open id `{}`: {}", id.to_string(), error);
            })?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).await?;
        let model: E = self.serializer_instance.from_slice(&data)?;
        return Ok(model);
    }

    pub async fn get_many(&self, ids: &[E::Id]) -> Result<Vec<E>, Failure> {
        let mut result = Vec::with_capacity(ids.len());
        for id in ids.iter() {
            let item = self.get(id).await?;
            result.push(item);
        }
        return Ok(result);
    }

    pub async fn find(&self, id: &E::Id) -> Result<Option<E>, Failure> {
        let path = RelativePath::from_string(id.to_string());
        let path = self.path_resolver.resolve_path(path)?;
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

    pub async fn is_exists(&self, id: &E::Id) -> Result<bool, Failure> {
        let path = RelativePath::from_string(id.to_string());
        let path = self.path_resolver.resolve_path(path)?;
        let async_path = AsyncPath::new(&path);
        let is_exists = async_path.exists().await;
        return Ok(is_exists);
    }

    pub async fn find_many(&self, ids: &[E::Id]) -> Result<Vec<Option<E>>, Failure> {
        let mut result = Vec::with_capacity(ids.len());
        for id in ids.iter() {
            let opt_item = self.find(id).await?;
            result.push(opt_item);
        }
        return Ok(result);
    }

    pub async fn store(&self, model: &E) -> Result<(), Failure> {
        let id = model.get_entity_id();
        let path = RelativePath::from_string(id.to_string());
        let path = self.path_resolver.resolve_path(path)?;
        create_parent_dir(&path).await?;
        let mut file = File::create(&path).await?;
        let data = self.serializer_instance.to_vec(&model)?;
        file.write_all(&data).await?;
        return Ok(());
    }

    pub async fn query<Q>(&self, query: Q) -> Result<EntityStream<'_, E>, Failure>
        where 
            Q: Query,
            Q: Send + Sync,
            E: Send + Sync + 'static,
    {
        let query_comparator = match self.query_comparator {
            Some(ref query_comparator) => query_comparator,
            None => {
                return crate::fail!("Query comparator not available");
            },
        };
        let entites_stream = self.all().await?;
        let entites_stream = entites_stream.filter_map(move |entity| {
            let mut result = None;
            if let Ok(entity) = entity {
                let is_match = query_comparator.compare_entity(&query, &entity);
                match is_match {
                    Ok(true) => {
                        result = Some(Ok(entity));
                    },
                    Ok(false) => {
                        result = None;
                    },
                    Err(error) => {
                        result = Some(Err(error));
                    },
                }
            }
            async move {
                return result;
            }
        });
        let entites_stream = EntityStream::new(entites_stream);
        return Ok(entites_stream);
    }

    pub async fn all(&self) -> Result<EntityStream<'_, E>, Failure> 
        where 
            E: Send + Sync + 'static,
    {
        let base_path = self.path_resolver.base_path()?;
        async_std::fs::create_dir_all(&base_path).await?;
        let walkdir_stream = WalkDir::new(base_path)
            .filter(|entry| {
                return async move {
                    let filetype = entry.file_type().await;
                    let filetype = match filetype {
                        Ok(filetype) => filetype,
                        Err(_) => {
                            return Filtering::Ignore;
                        },
                    };
                    if filetype.is_file() {
                        return Filtering::Continue;
                    }
                    return Filtering::Ignore;
                }
            });
        let walkdir_stream = walkdir_stream
            .then(async move |file_path| -> Result<E, Failure> {
                let file_path = file_path
                .map(|file_path| {
                    return file_path.path();
                })
                .map_err(|error| {
                    let error: Failure = error.into();
                    return error;
                });
                let file_path = file_path?;
                let mut file = File::open(&file_path).await?;
                let mut data: Vec<u8> = Vec::new();
                file.read_to_end(&mut data).await?;
                let model: E = self.serializer_instance.from_slice(&data)?;
                return Ok(model);
            });
        let walkdir_stream = EntityStream::new(walkdir_stream);
        return Ok(walkdir_stream);
    }
}