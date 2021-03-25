use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::model::inline_keyboard::InlineKeyboard;
use crate::telegram::action::fund_report_list_action::FundReportListAction;
use crate::telegram::model::callback_button::CallbackButton;
use crate::telegram::model::button::Button;
use crate::telegram::views::pager_keyboard_view::pager_keyboard_view;
use crate::telegram::views::date_view::date_view;

pub fn fund_report_list_view(action: &FundReportListAction) -> View {
    let mut view = View::new();
    let message = format!("<b>Список всех отчётов компании {}:</b>", action.get_company_name());
    let message = OutgoingMessage::update(
        action.get_outgoing_message_id().clone(),
        message,
    );
    let mut keyboard = InlineKeyboard::new();
    for fund_report_record in action.iter() {
        let view_button = CallbackButton::new(
            date_view(fund_report_record.get_fund_report_id().get_date()),
            fund_report_record.get_route_view().clone(),
        );
        let view_button = Button::CallbackButton(view_button);
        keyboard.push_single_button(view_button);
    }
    let pager_buttons = pager_keyboard_view(action.get_pager());
    keyboard.push_keyboard_line(pager_buttons);
    let message = message.with_reply_markup(keyboard);
    view.push_message(message);
    return view;
}