use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::model::button::Button;
use crate::telegram::model::inline_keyboard::InlineKeyboard;
use crate::telegram::action::subscription_list_action::SubscriptionListAction;
use crate::telegram::model::callback_button::CallbackButton;
use crate::telegram::views::pager_keyboard_view::pager_keyboard_view;

pub fn subscription_list_view(action: &SubscriptionListAction) -> View {
    let mut view = View::new();
    let message;
    if action.has_subscriptions() {
        message = "<b>Список фондов, на которые вы подписаны:</b>";
    } else {
        message = "<b>Вы еще не подписались ни на один фонд.</b>\nНажмите на /funds чтобы вывести список фондов.";
    }
    let message = OutgoingMessage::update(action.get_outgoing_message_id().clone(), message.to_string());
    let mut keyboard = InlineKeyboard::new();
    for fund_record in action.iter() {
        if fund_record.is_subscribed() {
            let text = fund_record.get_company_name().to_string();
            let view_button = CallbackButton::new(
                text,
                fund_record.get_route_view().clone(),
            );
            let view_button = Button::CallbackButton(view_button);
            let unsubscribe_button = CallbackButton::new(
                "❌ Отписаться".to_string(),
                fund_record.get_route_unsubscribe().clone(),
            );
            let unsubscribe_button = Button::CallbackButton(unsubscribe_button);
            keyboard.push_keyboard_line(vec![view_button, unsubscribe_button]);
        }
    }
    let pager_buttons = pager_keyboard_view(action.get_pager());
    keyboard.push_keyboard_line(pager_buttons);
    let message = message.with_reply_markup(keyboard);
    view.push_message(message);
    return view;
}