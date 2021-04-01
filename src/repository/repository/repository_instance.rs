use crate::prelude::*;
use crate::repository::model::entity::Entity;
use crate::repository::model::query::Query;
use crate::repository::repository::file_repository::FileRepository;
use crate::repository::model::entity_stream::EntityStream;

pub enum RepositoryInstance<E> {
    FileRepository(FileRepository<E>),
}

impl <E> RepositoryInstance<E>
    where 
        E: Entity,
{
    pub async fn get(&self, id: &E::Id) -> Result<E, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.get(id).await;
            },
        }
    }

    pub async fn get_many(&self, ids: &[E::Id]) -> Result<Vec<E>, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.get_many(ids).await;
            },
        }
    }

    pub async fn find(&self, id: &E::Id) -> Result<Option<E>, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.find(id).await;
            },
        }
    }

    pub async fn is_exists(&self, id: &E::Id) -> Result<bool, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.is_exists(id).await;
            },
        }
    }

    pub async fn find_many(&self, ids: &[E::Id]) -> Result<Vec<Option<E>>, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.find_many(ids).await;
            },
        }
    }

    pub async fn store(&self, model: &E) -> Result<(), Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.store(model).await;
            },
        }
    }

    pub async fn query<Q>(&self, query: Q) -> Result<EntityStream<'_, E>, Failure>
        where 
            Q: Query,
            Q: Send + Sync,
            E: Send + Sync + 'static,
    {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.query(query).await;
            },
        }
    }

    pub async fn all(&self) -> Result<EntityStream<'_, E>, Failure> 
        where 
            E: Send + Sync + 'static,
    {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.all().await;
            },
        }
    }
}