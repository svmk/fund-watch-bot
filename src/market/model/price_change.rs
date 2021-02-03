use crate::market::model::price::Price;

#[derive(Debug, Clone)]
pub struct PriceChange {
    from: Price,
    to: Price,
}

impl PriceChange {
    pub fn new(
        from: Price,
        to: Price,
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