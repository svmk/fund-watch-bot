use crate::fetching::model::url::Url;
use crate::prelude::*;

pub trait ApiRequest {
    fn url(&self, base_url: &Url) -> Result<Url, Failure>;
}