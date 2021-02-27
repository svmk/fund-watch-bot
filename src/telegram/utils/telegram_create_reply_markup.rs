use crate::telegram::model::reply_markup::ReplyMarkup;
use tbot::types::keyboard::inline::Button;
use tbot::types::keyboard::inline::Keyboard;
use tbot::types::keyboard::Any;

pub fn telegram_create_reply_markup(reply_markup: &ReplyMarkup) -> Option<Any> {
    match reply_markup {
        ReplyMarkup::InlineKeyboard(ref keyboard) => {
            unimplemented!()
            // return Some(Keyboard::new(&[]));
        },
        ReplyMarkup::None => {
            return None;
        },
    }
}