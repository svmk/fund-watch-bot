use crate::app::model::date::Date;
use crate::prelude::*;
use std::iter::Iterator;

pub struct DateIterator {
    current: Date,
    end: Date,
}

impl DateIterator {
    pub fn new(started_at: Date, ended_at: Date) -> Result<DateIterator, Failure> {
        if started_at > ended_at {
            return Err(Failure::msg("Year quartal iterator: started_at cannot be greater than ended_at."));
        }
        return Ok(DateIterator {
            current: started_at,
            end: ended_at,
        })
    }
}

impl Iterator for DateIterator {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let current = self.current.clone();
            self.current = self.current.next();
            return Some(current);
        }
        return None;
    }
}