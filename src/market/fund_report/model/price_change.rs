use crate::market::common::model::historical_price::HistoricalPrice;

#[derive(Debug, Clone)]
pub struct PriceChange {
    from: HistoricalPrice,
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