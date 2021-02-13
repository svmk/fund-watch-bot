use crate::sec_gov::model::form_13f::Form13F;
use crate::sec_gov::model::form_13f_componenttable::Form13FComponentTable;

#[derive(new, Debug)]
pub struct CompanyReport13F {
    form_13f: Form13F,
    information_table: Form13FComponentTable,
}