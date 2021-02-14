use crate::app::model::timestamp::TimeStamp;
use std::num::NonZeroU32;

#[derive(Debug, serde_query::Deserialize)]
pub struct ChartSplit {
    #[query(".commit1")]
    pub date: TimeStamp,
    #[query(".commit2")]
    pub numerator: NonZeroU32,
    #[query(".commit3")]
    pub denominator: NonZeroU32,
}