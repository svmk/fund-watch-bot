use crate::market::fund_report::model::weight::Weight;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct WeightChange {
    #[serde(rename = "from")]
    from: Weight,
    #[serde(rename = "to")]
    to: Weight,
}

impl WeightChange {
    pub fn is_buy(&self) -> bool {
        return self.to > self.from;
    }

    pub fn compute_buy(&self) -> Option<Weight> {
        if self.is_buy() {
            let share = self.to.sub(&self.from);
            return Some(share);
        }
        return None;
    }

    pub fn is_sell(&self) -> bool {
        return self.to < self.from;
    }

    pub fn compute_sell(&self) -> Option<Weight> {
        if self.is_sell() {
            let share = self.from.sub(&self.to);
            return Some(share);
        }
        return None;
    }
}