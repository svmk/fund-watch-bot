use crate::telegram::model::callback_button::CallbackButton;

#[derive(Debug, Clone)]
pub enum Button {
    CallbackButton(CallbackButton),
}