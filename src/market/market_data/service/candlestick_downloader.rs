use crate::repository::repository::repository_instance::RepositoryInstance;


use crate::market::market_data::model::company_price::CompanyPrice;
use crate::market::market_data::model::quartal_price_id::QuartalPriceId;
use crate::market::market_data::model::quartal_price::QuartalPrice;
use crate::market::market_data::model::chart_period::ChartPeriod;
use crate::market::market_data::error::candlestick_fetch_error::CandlestickFetchError;
use crate::yahoo_finance::service::yahoo_api::YahooApi;
use crate::yahoo_finance::model::chart::chart_request::ChartRequest;
use crate::yahoo_finance::model::common_api::interval::Interval;
use crate::app::model::timestamp::TimeStamp;

use typed_di::service::service::Service;
mod candlestick_request;
pub use self::candlestick_request::CandlestickRequest;
mod update_decision;
use self::update_decision::UpdateDecision;

#[derive(new)]
pub struct CandlestickDownloader {
    yahoo_api: Service<YahooApi>,
    ticker_price_repository: Service<RepositoryInstance<CompanyPrice>>,
    quartal_price_repository: Service<RepositoryInstance<QuartalPrice>>,
}

impl CandlestickDownloader {
    pub async fn fetch_by_ticker(&self, request: &CandlestickRequest) -> Result<(), CandlestickFetchError> {
        let mut ticker_price = self.fetch_ticker_price(request).await?;
        let quartal_price_ids_iterator = ticker_price.as_ref().iter_quartal_price_ids()?;
        let quartal_price_ids_iterator = quartal_price_ids_iterator.filter(|quartal_price_id| {
            return request.get_chart_period().intersects_year_quartal(quartal_price_id.get_period());
        });
        let mut quartal_prices = Vec::new();
        for quartal_price_id in quartal_price_ids_iterator {
            let quartal_price = self.fetch_quartal_price(&ticker_price, quartal_price_id).await?;
            quartal_prices.push(quartal_price);
        }
        self.update_ticker_chart(&mut ticker_price, &quartal_prices);
        // Saving changes.
        for quartal_price in quartal_prices {
            if quartal_price.is_need_update() {
                self.quartal_price_repository.store(quartal_price.as_ref()).await?;
            }
        }
        if ticker_price.is_need_update() {
            self.ticker_price_repository.store(ticker_price.as_ref()).await?;
        }
        if !ticker_price.get_actual_chart_period().is_actual(request.get_chart_period()) {
            return Err(CandlestickFetchError::CompanyNotAvailable(ticker_price.get_company_id().clone()));
        }
        return Ok(());
    }

    async fn fetch_ticker_price(&self, request: &CandlestickRequest) -> Result<UpdateDecision<CompanyPrice>, CandlestickFetchError> {
        let ticker_price = self
            .ticker_price_repository
            .find(request.get_company_id()).await?;
        let (is_acutal, mut ticker_price) = match ticker_price {
            Some(ticker_price) => {
                let is_acutal = ticker_price
                    .get_actual_chart_period()
                    .is_actual(request.get_chart_period());
                (is_acutal, ticker_price)
            },
            None => {(false, CompanyPrice::new(request.get_company_id().clone()))},
        };
        if !is_acutal {
            let ticker = request.get_company_id().get_opt_ticker();
            let ticker = match ticker {
                Some(ticker) => ticker,
                None => {
                    return Err(CandlestickFetchError::CompanyNotAvailable(request.get_company_id().clone()));
                },
            };
            let chart_request = ChartRequest::new(
                ticker.clone(), 
                Interval::ThreeMonths, 
                TimeStamp::zero(),
                TimeStamp::now(),
            );
            let chart_response = self.yahoo_api.send(chart_request).await?;
            let chart_response = chart_response.ok_or_else(|| {
                return CandlestickFetchError::CompanyNotAvailable(request.get_company_id().clone());
            })?;
            let chart_response = chart_response.get_charts()?;
            for split in chart_response.get_splits()? {
                if ticker_price.can_add_split(&split) {
                    ticker_price.add_split(split)?;
                }
            }
            let chart_period = chart_response.get_chart_period()?;
            ticker_price.update_chart_period(chart_period);
            return Ok(UpdateDecision::update(ticker_price));
        }
        return Ok(UpdateDecision::nothing(ticker_price));
    }

    fn update_ticker_chart(&self, ticker_price: &mut UpdateDecision<CompanyPrice>, quartal_prices: &[UpdateDecision<QuartalPrice>]) {
        for quartal_price in quartal_prices.iter() {
            if let Some(candlestick) = quartal_price.quartal_candlestick() {
                if ticker_price.need_update_chart_price(quartal_price.get_id(), &candlestick) {
                    ticker_price.as_mut().update_chart_price(quartal_price.get_id(), candlestick);
                }
            }
        }
    }

    async fn fetch_quartal_price(&self, ticker_price: &CompanyPrice, quartal_price_id: QuartalPriceId) -> Result<UpdateDecision<QuartalPrice>, CandlestickFetchError> {
        let quartal_price = self
            .quartal_price_repository
            .find(&quartal_price_id).await?;
        let (is_actual, mut quartal_price) = match quartal_price {
            Some(quartal_price) => {
                let is_actual = quartal_price.is_actual();
                (is_actual, quartal_price)
            },
            None => {
                let quartal_price = QuartalPrice::new(quartal_price_id.clone());
                (false, quartal_price)
            },
        };
        if !is_actual {
            let started_at = quartal_price.get_id().get_period().get_start().to_timestamp();
            let next_period = quartal_price.get_id().get_period().next();
            let ended_at = next_period.get_end().to_timestamp();
            let ticker = quartal_price.get_id().get_company_id().get_opt_ticker();
            let ticker = match ticker {
                Some(ticker) => ticker,
                None => {
                    return Err(CandlestickFetchError::CompanyNotAvailable(quartal_price.get_id().get_company_id().clone()));
                },
            };
            let chart_request = ChartRequest::new(
                ticker.clone(), 
                Interval::OneDay, 
                started_at,
                ended_at,
            );
            let chart_response = self.yahoo_api.send(chart_request).await?;
            let chart_response = chart_response.ok_or_else(|| {
                return crate::error!("Unable to fetch quartal price `{}`", quartal_price_id);
            })?;
            let chart_response = chart_response.get_charts()?;
            let candlesticks = chart_response.get_candlesticks()?;
            let candlesticks = ticker_price.calculate_original_candlesticks(candlesticks)?;
            let chart_period = ChartPeriod::from_year_quartal(quartal_price.get_id().get_period());
            for candlestick in candlesticks {
                if chart_period.contains_datetime(candlestick.get_timestamp()) {
                    let date = candlestick.get_timestamp().to_date();
                    quartal_price.update_chart_price(&date, candlestick);
                } else if candlestick.get_timestamp() > chart_period.get_end() {
                    quartal_price.mark_actual();
                }
            }            
            return Ok(UpdateDecision::update(quartal_price));
        }
        return Ok(UpdateDecision::nothing(quartal_price));
    }
}
