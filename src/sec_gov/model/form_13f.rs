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