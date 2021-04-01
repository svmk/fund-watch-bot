
use typed_di::service::service_id_resolver::ServiceIdResolver;
use typed_di::service::service_id::ServiceId;
use typed_di::argument::argument_id_resolver::ArgumentIdResolver;
use typed_di::async_di::container_declaration::ContainerDeclaration;
use typed_di::error::Error;
use crate::system::{app_config::AppConfig, di};
use crate::market::market_data::service::candlestick_downloader::CandlestickDownloader;
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::repository::file_repository::FileRepository;
use crate::serializer::service::json_serializer::JsonSerializer;


use crate::market::market_data::model::company_price::CompanyPrice;
use crate::market::market_data::model::quartal_price::QuartalPrice;

use crate::market::market_data::path_resolver::company_price_path_resolver::company_price_path_resolver;
use crate::market::market_data::path_resolver::quartal_price_path_resolver::quartal_price_path_resolver;


pub const CANDLESTICK_DOWNLOADER: ServiceId<CandlestickDownloader> = ServiceIdResolver::SERVICE_ID;
pub const CANDLESTICK_PROVIDER: ServiceId<CandlestickProvider> = ServiceIdResolver::SERVICE_ID;
pub const COMPANY_PRICE_REPOSITORY: ServiceId<RepositoryInstance<CompanyPrice>> = ServiceIdResolver::SERVICE_ID;
pub const QUARTAL_PRICE_REPOSITORY: ServiceId<RepositoryInstance<QuartalPrice>> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), Error> {
    builder.register(CANDLESTICK_DOWNLOADER, async move |resolver| {
        let service = CandlestickDownloader::new(
            resolver.get_service(di::yahoo_finance_di::YAHOO_API).await?,
            resolver.get_service(COMPANY_PRICE_REPOSITORY).await?,
            resolver.get_service(QUARTAL_PRICE_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    builder.register(CANDLESTICK_PROVIDER, async move |resolver| {
        let service = CandlestickProvider::new(
            resolver.get_service(CANDLESTICK_DOWNLOADER).await?,
            resolver.get_service(COMPANY_PRICE_REPOSITORY).await?,
            resolver.get_service(QUARTAL_PRICE_REPOSITORY).await?,
        );
        return Ok(service);
    })?;
    builder.register(COMPANY_PRICE_REPOSITORY, async move |resolver|{
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            company_price_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    builder.register(QUARTAL_PRICE_REPOSITORY, async move |resolver|{
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            quartal_price_path_resolver(path),
            JsonSerializer::new(),
        );
        return Ok(service);
    })?;
    return Ok(());
}