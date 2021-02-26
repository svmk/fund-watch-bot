use crate::prelude::*;
use crate::repository::model::identity::Identity;
use crate::repository::model::entity::Entity;
use crate::repository::model::query::Query;
use crate::repository::repository::file_repository::FileRepository;
use futures::stream::Stream;

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

    pub async fn find(&self, id: &I) -> Result<Option<E>, Failure> {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.find(id).await;
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

    pub async fn query<Q>(&self, query: Q) -> Result<impl Stream<Item=Result<E, Failure>> + '_, Failure>
        where 
            Q: Query,
            E: 'static,
    {
        match self {
            RepositoryInstance::FileRepository(ref service) => {
                return service.query(query).await;
            },
        }
    }
}