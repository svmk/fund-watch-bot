use crate::telegram::model::outgoing_message::OutgoingMessage;

#[derive(Debug, Clone)]
pub struct View {
    messages: Vec<OutgoingMessage>,
}

impl View {
    pub fn new() -> View {
        return View {
            messages: Vec::new(),
        }
    }

    pub fn with_one_message(message: impl Into<OutgoingMessage>) -> View {
        let mut result = View::new();
        result.push_message(message);
        return result;
    }

    pub fn push_message(&mut self, message: impl Into<OutgoingMessage>) {
        self.messages.push(message.into());
    }

    pub fn iter_messages(&self) -> impl Iterator<Item=&OutgoingMessage> {
        return self.messages.iter();
    }
}