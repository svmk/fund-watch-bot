use crate::sec_gov::model::cik::Cik;
use crate::sec_gov::model::company_name::CompanyName;
use crate::app::model::date::Date;

#[derive(new, Debug)]
pub struct Form13F {
    cik: Cik,
    company_name: CompanyName,
    period_of_report: Date,
    report_calendar_or_quarter: Date,
}

impl Form13F {
    pub fn get_period_of_report(&self) -> &Date {
        return &self.period_of_report;
    }
    
    pub fn get_company_name(&self) -> &CompanyName {
        return &self.company_name;
    }
}