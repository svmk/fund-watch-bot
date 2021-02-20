use crate::telegram::model::view::View;

pub fn start_view() -> View {
    let mut view = View::new();
    let text = "Этот бот предназначен для слежения за биржевыми фондами америки.";
    view.push_message(text);
    return view;
}