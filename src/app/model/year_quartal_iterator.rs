use crate::app::model::year_quartal::YearQuartal;
use crate::prelude::*;
use std::iter::Iterator;

pub struct YearQuartalIterator {
    current: YearQuartal,
    end: YearQuartal,
}

impl YearQuartalIterator {
    pub fn new(started_at: YearQuartal, ended_at: YearQuartal) -> Result<YearQuartalIterator, Failure> {
        if started_at > ended_at {
            return Err(Failure::msg("Year quartal iterator: started_at cannot be greater than ended_at."));
        }
        return Ok(YearQuartalIterator {
            current: started_at,
            end: ended_at,
        })
    }
}

impl Iterator for YearQuartalIterator {
    type Item = YearQuartal;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let current = self.current.clone();
            self.current = self.current.next();
            return Some(current);
        }
        return None;
    }
}