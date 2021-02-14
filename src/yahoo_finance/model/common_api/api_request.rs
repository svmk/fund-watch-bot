use crate::fetching::model::url::Url;
use crate::prelude::*;

pub trait ApiRequest {
    fn create_api_url(&self, base_url: &Url) -> Result<Url, Failure>;
}