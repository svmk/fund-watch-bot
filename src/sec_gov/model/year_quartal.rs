use crate::sec_gov::model::year::Year;
use crate::sec_gov::model::quartal::Quartal;

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