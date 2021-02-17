use crate::prelude::*;
use crate::app::model::date::Date;

#[derive(Debug, Clone, ValueObject, PartialEq, Eq, PartialOrd, Ord)]
#[value_object(error_type = "Failure", load_fn = "Year::from_i32")]
pub struct Year(i32);

impl Year {
    pub fn from_i32(year: i32) -> Result<Year, Failure> {
        return Ok(Year(year));
    }

    pub fn now() -> Year {
        let today = Date::today();
        return today.get_year();
    }
}