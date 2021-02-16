use crate::app::model::year::Year;
use crate::app::model::quartal::Quartal;
use std::fmt;

#[derive(new, Debug)]
pub struct YearQuartal {
    year: Year,
    quartal: Quartal,
}

impl YearQuartal {
    pub fn get_year(&self) -> &Year {
        return &self.year;
    }

    pub fn get_quartal(&self) -> &Quartal {
        return &self.quartal;
    }
}

impl fmt::Display for YearQuartal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}-{}", self.year, self.quartal);
    }
}