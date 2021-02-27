use crate::prelude::*;
use crate::telegram::model::chat::Chat;
use crate::telegram::query::chat_subscribed_to_fund_query::ChatSubscribedToFundQuery;

pub fn chat_subscribed_to_fund_query_handler(query: &ChatSubscribedToFundQuery, chat: &Chat) -> Result<bool, Failure> {
    let is_match = chat.is_subscribed(query.get_fund_id());
    return Ok(is_match);
}