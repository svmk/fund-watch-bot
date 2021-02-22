use crate::telegram::model::view::View;

pub fn unknown_command_view() -> View {
    let mut view = View::new();
    let text = "Неизвестная команда";
    view.push_message(text);
    return view;
}