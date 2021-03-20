use crate::prelude::*;
use crate::app::model::date::Date;

#[derive(Debug, Clone, ValueObject, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[value_object(error_type = "Failure", load_fn = "Year::from_i32")]
pub struct Year(i32);

impl Year {
    pub fn from_i32(year: i32) -> Result<Year, Failure> {
        return Ok(Year(year));
    }

    pub fn to_i32(self) -> i32 {
        return self.0;
    }

    pub fn now() -> Year {
        let today = Date::today();
        return today.get_year();
    }

    pub fn next(mut self) -> Year {
        self.0 += 1;
        return self;
    }
}