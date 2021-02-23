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