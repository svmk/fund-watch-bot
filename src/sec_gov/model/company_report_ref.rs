use crate::sec_gov::model::relative_url::RelativeUrl;
use crate::sec_gov::model::form_type::FormType;
use crate::sec_gov::model::cik::Cik;

#[derive(new, Debug)]
pub struct CompanyReportRef {
    cik: Cik,
    form_type: FormType,
    relative_url: RelativeUrl,
}

impl CompanyReportRef {
    pub fn get_relative_url(&self) -> &RelativeUrl {
        return &self.relative_url;
    }
}