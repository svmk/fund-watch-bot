use crate::sec_gov::model::form_13f::Form13F;
use crate::sec_gov::model::form_13f_componenttable::Form13FComponentTable;

#[derive(new, Debug)]
pub struct CompanyReport13F {
    form_13f: Form13F,
    information_table: Form13FComponentTable,
}

impl CompanyReport13F {
    pub fn get_form_13f(&self) -> &Form13F {
        return &self.form_13f;
    }
    
    pub fn get_information_table(&self) -> &Form13FComponentTable {
        return &self.information_table;
    }
}