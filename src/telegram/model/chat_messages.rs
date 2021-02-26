use crate::telegram::model::chat_id::ChatId;
use crate::telegram::model::message_id::MessageId;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;
use crate::repository::model::entity::Entity;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessages {
    #[serde(rename="chat_id")]
    chat_id: ChatId,
    #[serde(rename="messages")]
    messages: HashMap<OutgoingMessageId, MessageId>,
}

impl ChatMessages {
    pub fn new(chat_id: ChatId) -> ChatMessages {
        return ChatMessages {
            chat_id,
            messages: HashMap::new(),
        }
    }

    pub fn assign_message(&mut self, telegram_message_id: MessageId, outgoing_message_id: OutgoingMessageId) {
        let _ = self.messages.insert(outgoing_message_id, telegram_message_id);
    }

    pub fn get_telegram_message(&self, outgoing_message_id: &OutgoingMessageId) -> Option<MessageId> {
        return self.messages.get(outgoing_message_id).map(Clone::clone);
    }
}

impl Entity<ChatId> for ChatMessages {
    fn get_entity_id(&self) -> &ChatId {
        return &self.chat_id;
    }
}