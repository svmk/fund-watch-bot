use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::model::inline_keyboard::InlineKeyboard;
use crate::telegram::action::fund_report_info_action::FundReportInfoAction;
use crate::telegram::views::date_view::date_view;
use crate::telegram::views::pager_keyboard_view::pager_keyboard_view;
use crate::telegram::utils::text_table::{TextTable, Row};
use crate::telegram::views::company_id_view::company_id_view;

pub fn fund_report_info_view(action: &FundReportInfoAction) -> View {
    let report_date = date_view(action.get_report_date());
    let mut message = format!(
        "Позиции фонда <b>{}</b> на <b>{}</b>:\n", 
        action.get_fund_name(), 
        report_date,
    );
    let mut table = TextTable::new_empty();
    for component in action.iter() {
        let ticker = company_id_view(component.get_company_id());
        let price = match component.get_price() {
            Some(price) => {
                let price = price.into_f64();
                let price = format!("{:.2}", price);
                price
            },
            None => {
                "??.??".to_string()
            },
        };
        let volume = component.get_volume().into_f64();
        let volume = format!("{:.2}", volume);
        let weight = component.get_weight().clone().into_f64();
        let weight = format!("{:.2}%", weight);
        let row = Row::new()
            .with_code("<code>")
            .with_text(ticker)
            .with_text(price)
            .with_text(volume)
            .with_text(weight)
            .with_code("</code>");
        table = table.with_row(row);
    }
    message += format!("{}", table).as_str();
    let message = OutgoingMessage::update(action.get_outgoing_message_id().clone(), message);
    let mut keyboard = InlineKeyboard::new();
    let pager_buttons = pager_keyboard_view(action.get_pager());
    keyboard.push_keyboard_line(pager_buttons);
    let message = message.with_reply_markup(keyboard);
    return View::with_one_message(message);
}