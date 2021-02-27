use crate::telegram::model::inline_keyboard::InlineKeyboard;

#[derive(Debug)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboard),
    None,
}