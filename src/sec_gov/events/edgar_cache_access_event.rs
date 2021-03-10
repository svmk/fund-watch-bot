use crate::event_emitter::model::event::Event;
use crate::sec_gov::model::relative_url::RelativeUrl;

#[derive(new, Debug, Clone)]
pub struct EdgarCacheAccessEvent {
    url: RelativeUrl,
}

impl EdgarCacheAccessEvent {
    pub fn get_url(&self) -> &RelativeUrl {
        return &self.url;
    }
}

impl Event for EdgarCacheAccessEvent {}