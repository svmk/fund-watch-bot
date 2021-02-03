#[derive(Error, Debug)]
pub enum PubProxy {
    #[error("Pub proxy not found in response")]
    NotFoundInResponse,
}