use crate::sec_gov::model::year_quartal::YearQuartal;
use crate::sec_gov::model::year::Year;
use crate::sec_gov::model::relative_url::RelativeUrl;

#[derive(new, Debug)]
pub struct CompanyIndexRequest(YearQuartal);

impl CompanyIndexRequest {
    pub fn relative_path(&self) -> RelativeUrl {
        let path = format!(
            "{}/{}.idx",
            self.0.get_year(),
            self.0.get_quartal().display_long(),
        );
        return RelativeUrl::new(path);
    }

    pub fn is_cacheable(&self) -> bool {
        let now = Year::now();
        return self.0.get_year() != &now;
    }
}