use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::action::fund_list_action::FundListAction;
use crate::telegram::views::pager_view::pager_view;

pub fn fund_list_view(action: &FundListAction) -> View {
    let mut view = View::new();
    for fund_record in action.iter() {
        let mut message = format!("Фонд: {}\nCIK: {}", fund_record.get_company_name(), fund_record.get_fund_id());
        if fund_record.is_subscribed() {
            message += "\nВы подписаны на получение уведомлений.";
        }
        let message = OutgoingMessage::update(fund_record.get_outgoing_message_id().clone(), message);
        view.push_message(message);
    }
    let pager_message = pager_view(action.get_pager());
    view.push_message(pager_message);
    return view;
}