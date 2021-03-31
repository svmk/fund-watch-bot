use crate::market::common::model::company_id::CompanyId;

pub fn finviz_view(company_id: &CompanyId) -> (String, String) {
    if let Some(ticker) = company_id.get_opt_ticker() {
        let url = format!("<a href=\"https://finviz.com/quote.ashx?t={}\">finviz</a>", ticker);
        return ("finviz".to_string(), url);
    }
    return ("".to_string(), "".to_string());
}