use crate::fetching::model::url::Url;
use crate::prelude::*;

pub trait ApiRequest {
    fn url(&self) -> Result<Url, Failure>;
}