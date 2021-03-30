use crate::sec_gov::model::company_report_ref::CompanyReportRef;
use crate::app::model::date::Date;
use crate::repository::model::entity::Entity;
use std::collections::HashSet;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedReports {
    #[serde(rename="date")]
    date: Date,
    #[serde(rename="items")]
    items: HashSet<CompanyReportRef>,
}

impl ProcessedReports {
    pub fn new(date: Date) -> ProcessedReports {
        return ProcessedReports {
            date, 
            items: HashSet::new(),
        }
    }

    pub fn add_report_ref(&mut self, report_ref: CompanyReportRef) {
        let _ = self.items.insert(report_ref);
    }

    pub fn was_processed(&self, report_ref: &CompanyReportRef) -> bool {
        return self.items.contains(report_ref);
    }
}

impl Entity<Date> for ProcessedReports {
    fn get_entity_id(&self) -> &Date {
        return &self.date;
    }
}