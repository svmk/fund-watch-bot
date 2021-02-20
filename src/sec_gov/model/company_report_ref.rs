use crate::sec_gov::model::relative_url::RelativeUrl;
use crate::sec_gov::model::form_type::FormType;
use crate::sec_gov::model::cik::Cik;
use crate::app::model::date::Date;

#[derive(new, Debug, Clone)]
pub struct CompanyReportRef {
    cik: Cik,
    date: Date,
    form_type: FormType,
    relative_url: RelativeUrl,
}

impl CompanyReportRef {
    pub fn get_cik(&self) -> &Cik {
        return &self.cik;
    }

    pub fn get_date(&self) -> &Date {
        return &self.date;
    }
    
    pub fn get_relative_url(&self) -> &RelativeUrl {
        return &self.relative_url;
    }

    pub fn get_form_type(&self) -> &FormType {
        return &self.form_type;
    }
}