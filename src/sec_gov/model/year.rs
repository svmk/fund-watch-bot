use crate::prelude::*;
use crate::app::model::date::Date;

#[derive(Debug, ValueObject, PartialEq)]
#[value_object(error_type = "Failure", load_fn = "Year::from_u16")]
pub struct Year(u16);

impl Year {
    fn from_u16(year: u16) -> Result<Year, Failure> {
        return Ok(Year(year));
    }

    pub fn now() -> Year {
        let today = Date::today();
        let year = today.get_year().abs() as u16;
        return Year(year);
    }
}