use crate::telegram::model::inline_keyboard::InlineKeyboard;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum ReplyMarkup {
    #[serde(rename="inline")]
    InlineKeyboard(InlineKeyboard),
    #[serde(rename="none")]
    None,
}