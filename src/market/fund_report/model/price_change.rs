use crate::market::common::model::original_price::OriginalPrice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceChange {
    #[serde(rename = "from")]
    from: OriginalPrice,
    #[serde(rename = "to")]
    to: OriginalPrice,
}

impl PriceChange {
    pub fn new(
        from: OriginalPrice,
        to: OriginalPrice,
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