use crate::app::model::year::Year;
use crate::app::model::quartal::Quartal;
use crate::app::model::datetime::DateTime;
use std::fmt;

#[derive(new, Debug)]
pub struct YearQuartal {
    year: Year,
    quartal: Quartal,
}

impl YearQuartal {
    pub fn from_datetime(datetime: DateTime) -> YearQuartal {
        let year = datetime.get_year();
        let month = datetime.get_month();
        let quartal = Quartal::from_month(month);
        return YearQuartal {
            year,
            quartal,
        };
    }

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