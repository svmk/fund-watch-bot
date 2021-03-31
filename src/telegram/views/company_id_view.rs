use crate::market::common::model::company_id::CompanyId;

pub fn company_id_view(company_id: &CompanyId) -> String {
    match company_id.get_opt_ticker() {
        Some(ticker) => {
            return format!("{}", ticker);
        },
        None => {
            return format!("{}", company_id.get_cusip());
        },
    }
}