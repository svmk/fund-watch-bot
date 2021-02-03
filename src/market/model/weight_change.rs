use crate::market::model::weight::Weight;

#[derive(Debug, Clone)]
pub struct WeightChange {
    from: Weight,
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