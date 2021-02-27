use crate::telegram::model::inline_keyboard::InlineKeyboard;

#[derive(Debug, Clone)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboard),
    None,
}