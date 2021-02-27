use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::model::callback_button::CallbackButton;
use crate::telegram::model::button::Button;
use crate::telegram::model::inline_keyboard::InlineKeyboard;
use crate::telegram::action::pager_action::PagerAction;


pub fn pager_view(pager_action: &PagerAction) -> OutgoingMessage {
    let mut message = OutgoingMessage::update(
        pager_action.get_outgoing_message_id().clone(), 
        "".to_string(),
    );
    let mut buttons = Vec::new();
    for page in pager_action.iter_pages() {
        let text = format!("{}", page.get_number());
        let route = page.get_route().clone();
        let button = CallbackButton::new(text, route);
        let button = Button::CallbackButton(button);
        buttons.push(button);
    }
    let mut keyboard = InlineKeyboard::new();
    keyboard.push_keyboard_line(buttons);
    message = message.with_reply_markup(keyboard);
    return message;
}