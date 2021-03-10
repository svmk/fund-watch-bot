use crate::event_emitter::model::event::Event;
use crate::fetching::model::request::Request;

#[derive(new, Debug, Clone)]
pub struct SendRequestEvent {
    request: Request,
}

impl SendRequestEvent {
    pub fn get_request(&self) -> &Request {
        return &self.request;
    }
}

impl Event for SendRequestEvent {}