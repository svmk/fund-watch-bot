use crate::market::common::model::actual_price::ActualPrice;
use crate::app::model::timestamp::TimeStamp;

#[derive(Debug, serde_query::Deserialize)]
pub struct ChartDividend {
    #[query(".amount")]
    amount: ActualPrice,
    #[query(".date")]
    date: TimeStamp,
}