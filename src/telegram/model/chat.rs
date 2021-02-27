use crate::market::fund_report::model::fund_id::FundId;
use crate::telegram::model::chat_id::ChatId;
use crate::repository::model::entity::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    #[serde(rename="id")]
    id: ChatId,
    #[serde(rename="fund_subscriptions")]
    fund_subscriptions: Vec<FundId>,
}

impl Chat {
    pub fn new(id: ChatId) -> Chat {
        return Chat {
            id,
            fund_subscriptions: Vec::new(),
        }
    }

    pub fn get_fund_subscriptions(&self) -> &Vec<FundId> {
        return &self.fund_subscriptions;
    }

    pub fn is_subscribed(&self, fund_id: &FundId) -> bool {
        return self.fund_subscriptions.contains(&fund_id);
    }

    pub fn subscribe(&mut self, fund_id: FundId) {
        if !self.is_subscribed(&fund_id) {
            self.fund_subscriptions.push(fund_id);
        }
    }

    pub fn unsubscribe(&mut self, fund_id: &FundId) {
        self
            .fund_subscriptions
            .drain_filter(|item| {
                return item == fund_id;
            })
            .for_each(|_|{});
    }
}

impl Entity<ChatId> for Chat {
    fn get_entity_id(&self) -> &ChatId {
        return &self.id;
    }
}