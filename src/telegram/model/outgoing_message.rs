use crate::telegram::model::reply_markup::ReplyMarkup;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutgoingMessage {
    #[serde(rename="id")]
    id: OutgoingMessageId,
    #[serde(rename="text")]
    text: String,
    #[serde(rename="reply_markup")]
    reply_markup: ReplyMarkup,
    #[serde(rename="enable_notification")]
    enable_notification: bool,
}

impl OutgoingMessage {
    pub fn new(text: String) -> OutgoingMessage {
        return OutgoingMessage {
            id: OutgoingMessageId::new(),
            text,
            reply_markup: ReplyMarkup::None,
            enable_notification: false,
        }
    }
    
    pub fn update(id: OutgoingMessageId, text: String) -> OutgoingMessage {
        return OutgoingMessage {
            id,
            text,
            reply_markup: ReplyMarkup::None,
            enable_notification: false,
        }
    }

    pub fn with_reply_markup(mut self, reply_markup: impl Into<ReplyMarkup> + 'static) -> OutgoingMessage {
        self.reply_markup = reply_markup.into();
        return self;
    }

    pub fn get_id(&self) -> &OutgoingMessageId {
        return &self.id;
    }

    pub fn get_text(&self) -> &String {
        return &self.text;
    }

    pub fn get_reply_markup(&self) -> &ReplyMarkup {
        return &self.reply_markup;
    }

    pub fn is_notification_enabled(&self) -> bool {
        return self.enable_notification;
    }

    pub fn is_same(&self, other: &Self) -> bool {
        return self == other;
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