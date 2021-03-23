use crate::telegram::model::{message_id::MessageId, outgoing_message_id::OutgoingMessageId};
use crate::telegram::model::outgoing_message::OutgoingMessage;

#[derive(new, Debug, Serialize, Deserialize, Clone)]
pub struct SendedMessage {
    #[serde(rename="message_id")]
    message_id: MessageId,
    #[serde(rename="outgoing_message")]
    outgoing_message: OutgoingMessage,
}

impl SendedMessage {
    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return self.outgoing_message.get_id();
    }

    pub fn get_telegram_message_id(&self) -> MessageId {
        return self.message_id.clone();
    }

    pub fn need_update(&self, outgoing_message: &OutgoingMessage) -> bool {
        return !self.outgoing_message.is_same(outgoing_message);
    }
}