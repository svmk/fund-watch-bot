use crate::telegram::model::reply_markup::ReplyMarkup;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;

#[derive(Debug)]
pub struct OutgoingMessage {
    id: OutgoingMessageId,
    text: String,
    reply_markup: Option<ReplyMarkup>,
}

impl OutgoingMessage {
    pub fn new(text: String) -> OutgoingMessage {
        return OutgoingMessage {
            id: OutgoingMessageId::new(),
            text,
            reply_markup: None,
        }
    }

    pub fn get_id(&self) -> &OutgoingMessageId {
        return &self.id;
    }

    pub fn update(id: OutgoingMessageId, text: String) -> OutgoingMessage {
        return OutgoingMessage {
            id,
            text,
            reply_markup: None,
        }
    }

    pub fn get_text(&self) -> &String {
        return &self.text;
    }

    pub fn get_reply_markup(&self) -> &Option<ReplyMarkup> {
        return &self.reply_markup;
    }
}

impl From<String> for OutgoingMessage {
    fn from(text: String) -> Self {
        return OutgoingMessage::new(text);
    }
}

impl <'a>From<&'a str> for OutgoingMessage {
    fn from(text: &'a str) -> Self {
        return OutgoingMessage::new(text.to_string());
    }
}