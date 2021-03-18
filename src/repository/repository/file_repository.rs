use crate::prelude::*;
use crate::repository::model::identity::Identity;
use crate::repository::model::entity::Entity;
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
use std::path::PathBuf;
use std::sync::Arc;

pub struct FileRepository<I, E>
{
    _identity: PhantomData<I>,
    _entity: PhantomData<E>,
    path_resolver: PathResolver,
    serializer_instance: SerializerInstance,
    query_comparator: Service<QueryComparator>,
}

impl <I, E> FileRepository<I, E>
    where 
        I: Identity,
        E: Entity<I>,
{
    pub fn new(
        path_resolver: PathResolver,
        serializer_instance: SerializerInstance,
        query_comparator: Service<QueryComparator>,
    ) -> RepositoryInstance<I, E> {
        let repository = FileRepository {
            _identity: PhantomData{},
            _entity: PhantomData {},
            path_resolver,
            serializer_instance,
            query_comparator,
        };
        return RepositoryInstance::FileRepository(repository);
    }

    pub async fn get(&self, id: &I) -> Result<E, Failure> {
        let path = RelativePath::from_string(id.to_string());
        let path = self.path_resolver.resolve_path(path)?;
        let mut file = File::open(&path).await?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).await?;
        let model: E = self.serializer_instance.from_slice(&data)?;
        return Ok(model);
    }

    pub async fn get_many(&self, ids: &[I]) -> Result<Vec<E>, Failure> {
        let mut result = Vec::with_capacity(ids.len());
        for id in ids.iter() {
            let item = self.get(id).await?;
            result.push(item);
        }
        return Ok(result);
    }

    pub async fn find(&self, id: &I) -> Result<Option<E>, Failure> {
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

    pub async fn find_many(&self, ids: &[I]) -> Result<Vec<Option<E>>, Failure> {
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
            I: Send + Sync,
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
        let query = Arc::new(query);
        let walkdir_stream = walkdir_stream
            .filter_map(move |file_path| {
                let query = query.clone();
                return async move {
                    let file_path = file_path
                        .map(|file_path| {
                            return file_path.path();
                        })
                        .map_err(|error| {
                            let error: Failure = error.into();
                            return error;
                        });
                    let result = self.process_walkdir_stream(query.as_ref(), file_path).await;
                    return result.transpose();
                }
            });
        let walkdir_stream = EntityStream::new(walkdir_stream);
        return Ok(walkdir_stream);
    }

    async fn process_walkdir_stream<Q>(&self, query: &Q, file_path: Result<PathBuf, Failure>) -> Result<Option<E>, Failure>
        where 
            Q: Query,
            E: 'static,
    {
        let file_path = file_path?;
        let mut file = File::open(&file_path).await?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).await?;
        let model: E = self.serializer_instance.from_slice(&data)?;
        let model = match self.query_comparator.compare(query, &model)? {
            true => Some(model),
            false => None,
        };
        return Ok(model);
    }
}