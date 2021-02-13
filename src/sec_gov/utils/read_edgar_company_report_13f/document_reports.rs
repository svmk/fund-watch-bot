use crate::prelude::*;
use crate::sec_gov::model::form_13f_componenttable::Form13FComponentTable;
use crate::sec_gov::model::form_13f::Form13F;
use crate::sec_gov::model::company_report_13f::CompanyReport13F;

pub struct DocumentReports {
    form_13f: Option<Form13F>,
    information_table: Option<Form13FComponentTable>,
}

impl DocumentReports {
    pub fn new() -> DocumentReports {
        return DocumentReports {
            form_13f: None,
            information_table: None,
        };
    }

    pub fn set_form_13f(&mut self, value: Form13F) -> Result<(), Failure> {
        if self.form_13f.is_some() {
            return Err(Failure::msg("Edgar form 13F already parsed"));
        }
        self.form_13f = Some(value);
        return Ok(());
    }

    pub fn set_information_table(&mut self, value: Form13FComponentTable) -> Result<(), Failure> {
        if self.information_table.is_some() {
            return Err(Failure::msg("Edgar information table 13F already parsed"));
        }
        self.information_table = Some(value);
        return Ok(());
    }

    pub fn create_company_report_13f(self) -> Result<CompanyReport13F, Failure> {
        let form_13f = match self.form_13f {
            Some(form_13f) => form_13f,
            None => {
                return Err(Failure::msg("Edgar form 13F not parsed"));
            },
        };
        let information_table = match self.information_table {
            Some(information_table) => information_table,
            None => {
                return Err(Failure::msg("Edgar information table 13F not parsed"));
            }
        };
        let report = CompanyReport13F::new(form_13f, information_table);
        return Ok(report);
    }
}
