use crate::telegram::model::reply_markup::ReplyMarkup;

#[derive(Debug)]
pub struct OutgoingMessage {
    text: String,
    reply_markup: Option<ReplyMarkup>,
}

impl OutgoingMessage {
    pub fn new(text: String) -> OutgoingMessage {
        return OutgoingMessage {
            text,
            reply_markup: None,
        }
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