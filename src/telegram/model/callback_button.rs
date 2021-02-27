use crate::telegram::model::button::Button;
use crate::telegram::model::action_route::ActionRoute;

#[derive(new, Debug, Clone)]
pub struct CallbackButton {
    text: String,
    action_route: ActionRoute,
}

impl CallbackButton {
    pub fn get_text(&self) -> &String {
        return &self.text;
    }
    
    pub fn get_action_route(&self) -> &ActionRoute {
        return &self.action_route;
    }
}

impl Into<Button> for CallbackButton {
    fn into(self) -> Button {
        return Button::CallbackButton(self);
    }
}