use crate::prelude::*;

#[derive(Debug, Clone, ValueObject, PartialEq, Eq, PartialOrd, Ord)]
#[value_object(error_type = "Failure", load_fn = "Day::from_u32")]
pub struct Day(u32);

impl Day {
    pub const DAY_1: Day = Day(1);
    pub const DAY_30: Day = Day(30);
    pub const DAY_31: Day = Day(31);

    pub fn from_u32(day: u32) -> Result<Day, Failure> {
        if day >= 31 {
            return crate::fail!("Day cannot be `{}`", day);
        }
        return Ok(Day(day));
    }

    pub fn to_u32(self) -> u32 {
        return self.0;
    }
}