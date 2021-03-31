use crate::telegram::utils::text_table::{TextTable, Row};
use crate::telegram::action::fund_change_record::FundChangeRecord;
use crate::telegram::views::company_id_view::company_id_view;
use crate::telegram::views::finviz_view::finviz_view;

pub fn fund_change_table_view(sells: &[FundChangeRecord], buys: &[FundChangeRecord]) -> TextTable {
    let mut table = TextTable::new_empty();
    for component in sells.iter() {
        let ticker = company_id_view(component.get_company_id());
        let price = component.get_price().into_f64();
        let price = format!("{:.2}", price);
        let volume = component.get_share().into_f64();
        let volume = format!("{:.2}", volume);
        let weight = component.get_weight().clone().into_f64();
        let weight = format!("{:.2}%", weight);
        let finviz = finviz_view(component.get_company_id());
        let row = Row::new()
            .with_code("<code>")
            .with_text("BUY")
            .with_text(ticker)
            .with_text(price)
            .with_text(volume)
            .with_text(weight)
            .with_code("</code>")
            .with_cell(finviz.0, finviz.1);
        table = table.with_row(row);
    }
    for component in buys.iter() {
        let ticker = company_id_view(component.get_company_id());
        let price = component.get_price().into_f64();
        let price = format!("{:.2}", price);
        let volume = component.get_share().into_f64();
        let volume = format!("{:.2}", volume);
        let weight = component.get_weight().clone().into_f64();
        let weight = format!("{:.2}%", weight);
        let finviz = finviz_view(component.get_company_id());
        let row = Row::new()
            .with_code("<code>")
            .with_text("SELL")
            .with_text(ticker)
            .with_text(price)
            .with_text(volume)
            .with_text(weight)
            .with_code("</code>")
            .with_cell(finviz.0, finviz.1);
        table = table.with_row(row);
    }
    return table;
}