use crate::telegram::action::fund_change_record::FundChangeRecord;
use crate::market::fund_report::model::fund::Fund;
use crate::telegram::model::view::View;
use crate::telegram::model::outgoing_message::OutgoingMessage;
use crate::telegram::views::fund_change_table_view::fund_change_table_view;

pub fn fund_change_view(fund: &Fund, sells: &[FundChangeRecord], buys: &[FundChangeRecord]) -> View {
    let message = format!(
        "Уведомление о перебалансировки фонда <b>{}</b>:\n",
        fund.get_company_name(),
    );
    let table = fund_change_table_view(&sells, &buys);
    let message = format!("{}\n{}", message, table);
    let message = OutgoingMessage::new(message);
    return View::with_one_message(message);
}