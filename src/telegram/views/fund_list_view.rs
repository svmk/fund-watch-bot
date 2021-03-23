use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::action::fund_list_action::FundListAction;
use crate::telegram::views::pager_view::pager_view;

pub fn fund_list_view(action: &FundListAction) -> View {
    let mut view = View::new();
    if action.get_funds_count() == 0 {
        view.push_message("<b>Ничего не найдено.</b>");
    } else {
        view.push_message("<b>Список доступных фондов:</b>");
    }
    for fund_record in action.iter() {
        let mut message = format!("Фонд: {}\n", fund_record.get_company_name());
        if fund_record.is_subscribed() {
            message += format!("Отписаться: /unsubscribe@{}", fund_record.get_fund_id()).as_str();
        } else {
            message += format!("Подписаться: /subscribe@{}", fund_record.get_fund_id()).as_str();
        }
        let message = OutgoingMessage::update(fund_record.get_outgoing_message_id().clone(), message);
        view.push_message(message);
    }
    let pager_message = pager_view(action.get_pager());
    view.push_message(pager_message);
    return view;
}