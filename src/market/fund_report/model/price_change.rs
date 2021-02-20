use crate::market::common::model::historical_price::HistoricalPrice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceChange {
    #[serde(rename = "from")]
    from: HistoricalPrice,
    #[serde(rename = "to")]
    to: HistoricalPrice,
}

impl PriceChange {
    pub fn new(
        from: HistoricalPrice,
        to: HistoricalPrice,
    ) -> Option<PriceChange> {
        if from == to {
            return None;
        }
        return Some(PriceChange {
            from,
            to,
        });
    }
}