use crate::sec_gov::model::company_report_ref::CompanyReportRef;

#[derive(Debug)]
pub struct CompanyReportIndex {
    reports: Vec<CompanyReportRef>,
}

impl CompanyReportIndex {
    pub fn new() -> CompanyReportIndex {
        return CompanyReportIndex {
            reports: Vec::new(),
        };
    }

    pub fn push_report(&mut self, report: CompanyReportRef) {
        self.reports.push(report);
    }

    pub fn iter(&self) -> impl Iterator<Item=&CompanyReportRef> {
        return self.reports.iter();
    }
}