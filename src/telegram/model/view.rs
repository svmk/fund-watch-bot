use crate::telegram::model::outgoing_message::OutgoingMessage;

#[derive(Debug)]
pub struct View {
    messages: Vec<OutgoingMessage>,
}

impl View {
    pub fn new() -> View {
        return View {
            messages: Vec::new(),
        }
    }

    pub fn push_message(&mut self, message: impl Into<OutgoingMessage>) {
        self.messages.push(message.into());
    }

    pub fn iter_messages(&self) -> impl Iterator<Item=&OutgoingMessage> {
        return self.messages.iter();
    }
}