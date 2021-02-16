use crate::app::model::datetime::DateTime;
use crate::market::common::model::price::Price;
use crate::prelude::*;
use std::num::NonZeroU32;

#[derive(new, Debug)]
pub struct SplitRule {
    started_at: Option<DateTime>,
    ended_at: Option<DateTime>,
    nominator: NonZeroU32,
    denominator: NonZeroU32,
}

impl SplitRule {
    pub fn is_match_datetime(&self, datetime: &DateTime) -> bool {
        // started_at <= date < ended_at
        if let Some(ref started_at) = self.started_at {
            if datetime < started_at {
                return false;
            }   
        }
        if let Some(ref ended_at) = self.ended_at {
            if datetime >= ended_at {
                return false;
            }
        }
        return true;
    }

    pub fn calculate(&self, value: f64) -> f64 {
        let value = value / self.nominator.get() as f64;
        let value = value * self.denominator.get() as f64;
        return value;
    }
}