use crate::repository::repository::file_repository::FileRepository;

pub enum RepositoryInstance<I, E> {
    FileRepository(FileRepository<I, E>),
}