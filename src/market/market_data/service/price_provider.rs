use crate::yahoo_finance::service::yahoo_api::YahooApi;
use typed_di::service::Service;

#[derive(new)]
pub struct PriceProvider {
    yahoo_api: Service<YahooApi>,
}

impl PriceProvider {
    
}