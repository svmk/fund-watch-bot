use crate::market::common::model::share::Share;

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct ShareChange {
    #[serde(rename = "from")]
    from: Share,
    #[serde(rename = "to")]
    to: Share,
}

impl ShareChange {
    pub fn is_buy(&self) -> bool {
        return self.to > self.from;
    }

    pub fn compute_buy(&self) -> Option<Share> {
        if self.is_buy() {
            let share = self.to.sub(&self.from);
            return Some(share);
        }
        return None;
    }

    pub fn is_sell(&self) -> bool {
        return self.to < self.from;
    }

    pub fn compute_sell(&self) -> Option<Share> {
        if self.is_sell() {
            let share = self.from.sub(&self.to);
            return Some(share);
        }
        return None;
    }
}