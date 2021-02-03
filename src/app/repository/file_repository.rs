use crate::prelude::*;
use crate::app::model::identity::Identity;
use crate::app::model::entity::Entity;
use std::marker::PhantomData;
use std::path::PathBuf;

pub struct FileRepository<I, T>
{
    _identity: PhantomData<I>,
    _entity: PhantomData<T>,
    path: PathBuf,
}

impl <I, T> FileRepository<I, T>
    where 
        I: Identity,
        T: Entity<I>,
{
    pub fn new(path: PathBuf) -> Result<FileRepository<I, T>, Failure> {
        std::fs::create_dir_all(&path)?;
        return Ok(FileRepository {
            _identity: PhantomData{},
            _entity: PhantomData {},
            path,
        });
    }

    pub fn get(&self, id: &I) -> Result<T, Failure> {
        unimplemented!()
    }    
}