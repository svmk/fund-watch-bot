use crate::telegram::model::view::View;

pub fn start_view() -> View {
    let mut view = View::new();
    let text = "Привет! Этот бот следит за биржевыми фондами америки.\n\n/funds Список всех фондов";
    view.push_message(text);
    return view;
}