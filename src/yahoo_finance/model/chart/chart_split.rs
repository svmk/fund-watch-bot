use crate::app::model::timestamp::TimeStamp;
use crate::market::model::split::Split;
use std::num::NonZeroU32;

#[derive(Debug, serde_query::Deserialize)]
pub struct ChartSplit {
    #[query(".commit1")]
    date: TimeStamp,
    #[query(".commit2")]
    numerator: NonZeroU32,
    #[query(".commit3")]
    denominator: NonZeroU32,
}

impl ChartSplit {
    pub fn create_split(&self) -> Split {
        return Split::new(
            self.date.to_datetime(),
            self.numerator,
            self.denominator,
        );
    }
}