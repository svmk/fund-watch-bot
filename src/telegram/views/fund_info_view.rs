use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::model::inline_keyboard::InlineKeyboard;
use crate::telegram::action::fund_info_action::FundInfoAction;
use crate::telegram::model::callback_button::CallbackButton;
use crate::telegram::model::button::Button;

pub fn fund_info_view(action: &FundInfoAction) -> View {
    let fund = action.get_fund();
    let mut view = View::new();
    let message = format!(
        "Фонд: <b>{}</b>\nCIK: {}", 
        fund.get_company_name(), 
        fund.get_fund_id(),
    );
    let message = OutgoingMessage::update(
        action.get_outgoing_message_id().clone(), 
        message,
    );
    let mut keyboard = InlineKeyboard::new();
    let fund_reports_list_button = CallbackButton::new(
        "Отчёты".to_string(), 
        action.get_fund_report_list_route().clone(),
    );
    let fund_reports_list_button = Button::CallbackButton(fund_reports_list_button);
    let fund_changes_list_button = CallbackButton::new(
        "Перебалансировки".to_string(),
        action.get_fund_change_list_route().clone(),
    );
    let fund_changes_list_button = Button::CallbackButton(fund_changes_list_button);
    let subscribe_button= subscribe_button(action);
    keyboard.push_keyboard_line(vec![fund_reports_list_button, fund_changes_list_button, subscribe_button]);
    let message = message.with_reply_markup(keyboard);
    view.push_message(message);
    return view;
}


fn subscribe_button(action: &FundInfoAction) -> Button {
    let button;
    if action.is_subscribed() {
        let text = format!("Отписаться");
        button = CallbackButton::new(text, action.get_unsubscribe_route().clone());
    } else {
        let text = format!("Подписаться");
        button = CallbackButton::new(text, action.get_subscribe_route().clone());
    }
    return Button::CallbackButton(button);
}