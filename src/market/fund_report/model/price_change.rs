use crate::market::common::model::original_price::OriginalPrice;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct PriceChange {
    #[serde(rename = "from")]
    from: OriginalPrice,
    #[serde(rename = "to")]
    to: OriginalPrice,
}


impl PriceChange {
    pub fn get_to(&self) -> &OriginalPrice {
        return &self.to;
    }
    
    pub fn is_buy(&self) -> bool {
        return self.to > self.from;
    }

    pub fn compute_buy(&self) -> Option<OriginalPrice> {
        if self.is_buy() {
            let share = self.to.sub(&self.from);
            return Some(share);
        }
        return None;
    }

    pub fn is_sell(&self) -> bool {
        return self.to < self.from;
    }

    pub fn compute_sell(&self) -> Option<OriginalPrice> {
        if self.is_sell() {
            let share = self.from.sub(&self.to);
            return Some(share);
        }
        return None;
    }
}