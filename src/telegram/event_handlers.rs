use crate::event_emitter::model::event_handler_id::EventHandlerId;
use crate::market::fund_report::events::new_fund_change_event::NewFundChangeEvent;

pub const FUND_CHANGE_TELEGRAM_NOTIFICATION: EventHandlerId<NewFundChangeEvent> = EventHandlerId::from_uuid(0x78f36e1c_a815_4272_b808_005e9027dcd8);