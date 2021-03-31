use crate::telegram::utils::text_table::{TextTable, Row};
use crate::market::fund_report::model::fund_component_buy::FundComponentBuy;
use crate::market::fund_report::model::fund_component_sell::FundComponentSell;
use crate::telegram::views::company_id_view::company_id_view;

pub fn fund_change_table_view(sells: &[FundComponentSell], buys: &[FundComponentBuy]) -> TextTable {
    let mut table = TextTable::new_empty();
    for component in sells.iter() {
        let ticker = company_id_view(component.get_company_id());
        let price = component.get_sell_price().into_f64();
        let price = format!("{:.2}", price);
        let volume = component.get_sold_share().into_f64();
        let volume = format!("{:.2}", volume);
        let weight = component.get_sold_weight().clone().into_f64();
        let weight = format!("{:.2}%", weight);
        let row = Row::new()
            .with_code("<code>")
            .with_text("BUY")
            .with_text(ticker)
            .with_text(price)
            .with_text(volume)
            .with_text(weight)
            .with_code("</code>");
        table = table.with_row(row);
    }
    for component in buys.iter() {
        let ticker = company_id_view(component.get_company_id());
        let price = component.get_buy_price().into_f64();
        let price = format!("{:.2}", price);
        let volume = component.get_buyed_share().into_f64();
        let volume = format!("{:.2}", volume);
        let weight = component.get_buyed_weight().clone().into_f64();
        let weight = format!("{:.2}%", weight);
        let row = Row::new()
            .with_code("<code>")
            .with_text("SELL")
            .with_text(ticker)
            .with_text(price)
            .with_text(volume)
            .with_text(weight)
            .with_code("</code>");
        table = table.with_row(row);
    }
    return table;
}