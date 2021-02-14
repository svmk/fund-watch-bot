use crate::market::model::split_rule::SplitRule;
use crate::market::model::split::Split;
use crate::prelude::*;

#[derive(Debug)]
pub struct SplitRules {
    split_rules: Vec<SplitRule>,
    splits: Vec<Split>,
}

impl SplitRules {
    pub fn new() -> SplitRules {
        return SplitRules {
            split_rules: Vec::new(),
            splits: Vec::new(),
        };
    }

    pub fn can_add_split(&self, split: &Split) -> bool {
        if let Some(last_split) = self.splits.last() {
            return split.get_datetime() > last_split.get_datetime();
        }
        return true;
    }

    pub fn add_split(&mut self, split: Split) -> Result<(), Failure> {
        if !self.can_add_split(&split) {
            return Err(Failure::msg(format!("Split cannot be added for date {}", split.get_datetime())));
        }
        self.splits.push(split);
        return Ok(());
    }

    fn generate_split_rules(&mut self) {
        unimplemented!()
    }
}