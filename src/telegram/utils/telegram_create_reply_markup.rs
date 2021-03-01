use crate::telegram::model::{button::Button, inline_keyboard::InlineKeyboard, reply_markup::ReplyMarkup};
use tbot::types::keyboard::inline::Button as TgButton;
use tbot::types::keyboard::inline::Keyboard as TgKeyboard;
use tbot::types::keyboard::inline::Markup as TgMarkup;
use tbot::types::keyboard::inline::ButtonKind as TgButtonKind;
use std::borrow::Cow;

pub fn telegram_create_reply_markup(reply_markup: &ReplyMarkup) -> Option<TgKeyboard> {
    match reply_markup {
        ReplyMarkup::InlineKeyboard(ref keyboard) => {
            let keyboard = create_inline_keyboard(keyboard);
            return Some(keyboard);
        },
        ReplyMarkup::None => {
            return None;
        },
    }
}

fn create_inline_keyboard(keyboard: &InlineKeyboard) -> TgKeyboard {
    let mut markup = TgMarkup::new_empty();
    for buttons in keyboard.iter_markup() {
        let buttons: Vec<_> = buttons.iter().map(create_button).collect();
        markup.push_markup_line(buttons);
    }
    let keyboard = TgKeyboard::new(markup);
    return keyboard;
}

fn create_button(button: &Button) -> TgButton {
    match button {
        Button::CallbackButton(button) => {
            let text = Cow::Borrowed(button.get_text().as_str());
            let callback = button.get_action_route().to_string();
            let callback = Cow::Owned(callback);
            let button_type = TgButtonKind::CallbackData(callback);
            return TgButton::new(text, button_type);
        },
    }
}