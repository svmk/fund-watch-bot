use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::action::fund_change_info_action::FundChangeInfoAction;
use crate::telegram::views::fund_change_table_view::fund_change_table_view;

pub fn fund_change_info_view(action: &FundChangeInfoAction) -> View {
    let message = format!(
        "Перебалансировка фонда <b>{}</b>:\n",
        action.get_fund_name(),
    );
    let table = fund_change_table_view(action.get_sells(), action.get_buys());
    let message = format!("{}\n{}", message, table);
    let message = OutgoingMessage::update(action.get_outgoing_message_id().clone(), message);
    return View::with_one_message(message);
}