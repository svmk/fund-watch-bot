use crate::market::fund_report::model::weight::Weight;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightChange {
    #[serde(rename = "from")]
    from: Weight,
    #[serde(rename = "to")]
    to: Weight,
}

impl WeightChange {
    pub fn new(
        from: Weight,
        to: Weight,
    ) -> Option<WeightChange> {
        if from == to {
            return None;
        }
        return Some(WeightChange {
            from,
            to,
        });
    }
}