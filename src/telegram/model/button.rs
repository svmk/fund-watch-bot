use crate::telegram::model::callback_button::CallbackButton;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Button {
    CallbackButton(CallbackButton),
}