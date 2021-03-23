use crate::telegram::model::callback_button::CallbackButton;
use crate::telegram::model::button::Button;
use crate::telegram::action::pager_action::PagerAction;


pub fn pager_keyboard_view(pager_action: &PagerAction) -> Vec<Button> {
    let mut pager_buttons = Vec::new();
    for page in pager_action.iter_pages() {
        let text = format!("{}", page.get_page());
        let route = page.get_route().clone();
        let button = CallbackButton::new(text, route);
        let button = Button::CallbackButton(button);
        pager_buttons.push(button);
    }
    return pager_buttons;
}