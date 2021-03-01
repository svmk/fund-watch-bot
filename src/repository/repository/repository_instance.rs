use crate::prelude::*;
use crate::repository::model::identity::Identity;
use crate::repository::model::entity::Entity;
use crate::repository::model::query::Query;
use crate::repository::repository::file_repository::FileRepository;
use crate::repository::model::entity_stream::EntityStream;

pub enum RepositoryInstance<I, E> {
    FileRepository(FileRepository<I, E>),
}

impl <I, E> RepositoryInstance<I, E>
    where 
        I: Identity,
        E: Entity<I>,
{
    pub async fn get(&self, id: &I) -> Result<E, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.get(id).await;
            },
        }
    }

    pub async fn get_many(&self, ids: &[I]) -> Result<Vec<E>, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.get_many(ids).await;
            },
        }
    }

    pub async fn find(&self, id: &I) -> Result<Option<E>, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.find(id).await;
            },
        }
    }

    pub async fn find_many(&self, ids: &[I]) -> Result<Vec<Option<E>>, Failure> {
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
            I: Send + Sync,
            E: Send + Sync + 'static,
    {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.query(query).await;
            },
        }
    }
}