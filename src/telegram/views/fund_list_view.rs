use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::action::fund_list_action::FundListAction;
use crate::telegram::model::callback_button::CallbackButton;
use crate::telegram::model::button::Button;
use crate::telegram::model::inline_keyboard::InlineKeyboard;
use crate::telegram::views::pager_keyboard_view::pager_keyboard_view;

pub fn fund_list_view(action: &FundListAction) -> View {
    let mut view = View::new();
    let message;
    if action.get_funds_count() == 0 {
        message = "<b>Ничего не найдено.</b>";
    } else {
        message = "<b>Список доступных фондов:</b>";
    }
    let message = OutgoingMessage::update(action.get_outgoing_message_id().clone(), message.to_string());
    let mut keyboard = InlineKeyboard::new();
    for fund_record in action.iter() {
        let text = match fund_record.is_subscribed() {
            true => {
                format!("🔔 {}", fund_record.get_company_name())
            },
            false => {
                format!("{}", fund_record.get_company_name())
            },
        };
        let button = CallbackButton::new(
            text,
            fund_record.get_route_view().clone(),
        );
        let button = Button::CallbackButton(button);
        keyboard.push_single_button(button);
    }
    let pager_buttons = pager_keyboard_view(action.get_pager());
    keyboard.push_keyboard_line(pager_buttons);
    let message = message.with_reply_markup(keyboard);
    view.push_message(message);
    return view;
}