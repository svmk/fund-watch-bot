use crate::telegram::model::reply_markup::ReplyMarkup;
use crate::telegram::model::button::Button;

#[derive(Debug, Clone)]
pub struct InlineKeyboard {
    markup: Vec<Vec<Button>>,
}

impl InlineKeyboard {
    pub fn new() -> InlineKeyboard {
        return InlineKeyboard {
            markup: Vec::new(),
        }
    }

    pub fn push_keyboard_line(&mut self, buttons: Vec<Button>) {
        self.markup.push(buttons);
    }

    pub fn push_single_button(&mut self, button: impl Into<Button>) {
        self.push_keyboard_line(vec![button.into()]);
    }

    pub fn iter_markup(&self) -> impl Iterator<Item=&Vec<Button>> {
        return self.markup.iter();
    }
}

impl Into<ReplyMarkup> for InlineKeyboard {
    fn into(self) -> ReplyMarkup {
        return ReplyMarkup::InlineKeyboard(self);
    }
}