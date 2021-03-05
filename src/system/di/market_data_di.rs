use typed_di::sync_context::sync_container_declaration::SyncContainerDeclaration;
use typed_di::service_id_resolver::ServiceIdResolver;
use typed_di::service_id::ServiceId;
use typed_di::argument_id_resolver::ArgumentIdResolver;
use typed_di::container_declaration::ContainerDeclaration;
use typed_di::error::BuildError;
use crate::system::{app_config::AppConfig, di};
use crate::market::market_data::service::candlestick_downloader::CandlestickDownloader;
use crate::market::market_data::service::candlestick_provider::CandlestickProvider;
use crate::repository::repository::repository_instance::RepositoryInstance;
use crate::repository::repository::file_repository::FileRepository;
use crate::serializer::service::json_serializer::JsonSerializer;
use crate::market::common::model::ticker::Ticker;
use crate::market::market_data::model::ticker_price::TickerPrice;
use crate::market::market_data::model::quartal_price::QuartalPrice;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::day_price_id::DayPriceId;
use crate::market::market_data::model::day_price::DayPrice;
use crate::market::market_data::path_resolver::ticker_price_path_resolver::ticker_price_path_resolver;
use crate::market::market_data::path_resolver::quartal_price_path_resolver::quartal_price_path_resolver;
use crate::market::market_data::path_resolver::daily_price_path_resolver::daily_price_path_resolver;


pub const CANDLESTICK_DOWNLOADER: ServiceId<CandlestickDownloader> = ServiceIdResolver::SERVICE_ID;
pub const CANDLESTICK_PROVIDER: ServiceId<CandlestickProvider> = ServiceIdResolver::SERVICE_ID;
pub const TICKER_PRICE_REPOSITORY: ServiceId<RepositoryInstance<Ticker, TickerPrice>> = ServiceIdResolver::SERVICE_ID;
pub const QUARTAL_PRICE_REPOSITORY: ServiceId<RepositoryInstance<QuartalPriceId, QuartalPrice>> = ServiceIdResolver::SERVICE_ID;
pub const DAILY_PRICE_REPOSITORY: ServiceId<RepositoryInstance<DayPriceId, DayPrice>> = ServiceIdResolver::SERVICE_ID;

pub fn register_services(builder: &mut ContainerDeclaration) -> Result<(), BuildError> {
    builder.register(CANDLESTICK_DOWNLOADER, |resolver| {
        let service = CandlestickDownloader::new(
            resolver.get_service(di::yahoo_finance_di::YAHOO_API)?,
            resolver.get_service(TICKER_PRICE_REPOSITORY)?,
            resolver.get_service(QUARTAL_PRICE_REPOSITORY)?,
            resolver.get_service(DAILY_PRICE_REPOSITORY)?,
        );
        return Ok(service);
    })?;
    builder.register(CANDLESTICK_PROVIDER, |resolver| {
        let service = CandlestickProvider::new(
            resolver.get_service(CANDLESTICK_DOWNLOADER)?,
            resolver.get_service(QUARTAL_PRICE_REPOSITORY)?,
            resolver.get_service(DAILY_PRICE_REPOSITORY)?,
        );
        return Ok(service);
    })?;
    builder.register(TICKER_PRICE_REPOSITORY, |resolver|{
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            ticker_price_path_resolver(path),
            JsonSerializer::new(),
            resolver.get_service(di::repository_di::QUERY_COMPARATOR)?,
        );
        return Ok(service);
    })?;
    builder.register(QUARTAL_PRICE_REPOSITORY, |resolver|{
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            quartal_price_path_resolver(path),
            JsonSerializer::new(),
            resolver.get_service(di::repository_di::QUERY_COMPARATOR)?,
        );
        return Ok(service);
    })?;
    builder.register(DAILY_PRICE_REPOSITORY, |resolver|{
        let config = resolver.get_argument(AppConfig::ARGUMENT_ID)?;
        let config = config.get_repository();
        let path = config.get_path();
        let service = FileRepository::new(
            daily_price_path_resolver(path),
            JsonSerializer::new(),
            resolver.get_service(di::repository_di::QUERY_COMPARATOR)?,
        );
        return Ok(service);
    })?;
    return Ok(());
}