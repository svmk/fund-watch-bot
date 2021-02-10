use crate::prelude::*;
use crate::sec_gov::model::company_report_ref::CompanyReportRef;
use crate::sec_gov::model::relative_url::RelativeUrl;
use crate::sec_gov::model::form_type::FormType;
use crate::sec_gov::model::cik::Cik;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct IndexTableRecord {
    values: HashMap<String, String>,
}

impl IndexTableRecord {
    const CIK: &'static str = "CIK";
    const FORM_TYPE: &'static str = "Form Type";
    const FILE_NAME: &'static str = "File Name";

    pub fn new() -> IndexTableRecord {
        return IndexTableRecord {
            values: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: String, value: String) {
        let _ = self.values.insert(key, value);
    }
    
    fn get_field(&self, key: &str) -> Result<String, Failure> {
        if let Some(value) = self.values.get(key) {
            return Ok(value.clone());
        }
        return Err(Failure::msg(format!("Unable to get key `{}` from edgar index record", key)));
    }

    pub fn create_company_report_ref(&self) -> Result<CompanyReportRef, Failure> {
        let cik = self.get_field(Self::CIK)?;
        let cik = Cik::from_str(&cik)?;
        let form_type = self.get_field(Self::FORM_TYPE)?;
        let form_type = FormType::from_string(form_type)?;
        let relative_url = self.get_field(Self::FILE_NAME)?;
        let relative_url = RelativeUrl::from_string(relative_url)?;
        let result = CompanyReportRef::new(
            cik,
            form_type,
            relative_url,
        );
        return Ok(result);
    }
}
