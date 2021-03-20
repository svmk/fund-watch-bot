use crate::app::model::datetime::DateTime;
use crate::app::model::year_quartal::YearQuartal;
#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct ChartPeriod {
    #[serde(rename="start")]
    start: DateTime,
    #[serde(rename="end")]
    end: DateTime,
}

impl ChartPeriod {
    pub fn from_year_quartal(year_quartal: &YearQuartal) -> ChartPeriod {
        return ChartPeriod {
            start: year_quartal.get_start(),
            end: year_quartal.get_end(),
        }
    }
    
    pub fn get_start(&self) -> &DateTime {
        return &self.start;
    }

    pub fn get_end(&self) -> &DateTime {
        return &self.end;
    }

    pub fn contains_datetime(&self, other: &DateTime) -> bool {
        if other < self.get_start() {
            return false;
        }
        if other > self.get_end() {
            return false;
        }
        return true;
    }

    pub fn intersects_year_quartal(&self, other: &YearQuartal) -> bool {
        let start = other.get_start();
        if self.contains_datetime(&start) {
            return true;
        }
        let end = other.get_end();
        if self.contains_datetime(&end) {
            return true;
        }
        return false;
    }
    
    pub fn contains(&self, other: &Self) -> bool {
        if !self.contains_datetime(other.get_start()) {
            return false;
        }
        if !self.contains_datetime(other.get_end()) {
            return false;
        }
        return true;
    }
}