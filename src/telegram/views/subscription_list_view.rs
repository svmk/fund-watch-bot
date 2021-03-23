use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::model::inline_keyboard::InlineKeyboard;
use crate::telegram::action::fund_list_action::FundListAction;
use crate::telegram::model::callback_button::CallbackButton;

pub fn subscription_list_view(action: &FundListAction) -> View {
    unimplemented!()
    // let mut view = View::new();
    // if action.get_funds_count() == 0 {
    //     view.push_message("Ничего не найдено.");
    // } else {
    //     view.push_message("Список ваших подписок:");
    // }
    // for fund_record in action.iter() {
    //     let message = format!("Фонд: {}\nCIK: {}", fund_record.get_company_name(), fund_record.get_fund_id());
    //     let mut message = OutgoingMessage::update(fund_record.get_outgoing_message_id().clone(), message);
    //     let mut keyboard = InlineKeyboard::new();
    //     if fund_record.is_subscribed() {
    //         let button = CallbackButton::new(
    //             "Отписаться".to_string(),
    //             fund_record.get_route_unsubscribe().clone(),
    //         );
    //         keyboard.push_single_button(button);
    //     } else {
    //         let button = CallbackButton::new(
    //             "Подписаться".to_string(),
    //             fund_record.get_route_subscribe().clone(),
    //         );
    //         keyboard.push_single_button(button);
    //     }
    //     message = message.with_reply_markup(keyboard);
    //     view.push_message(message);
    // }
    // let pager_message = pager_view(action.get_pager());
    // view.push_message(pager_message);
    // return view;
}