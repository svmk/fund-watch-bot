use crate::prelude::*;
use crate::event_emitter::model::event::Event;
use crate::event_emitter::model::packed_event::PackedEvent;
use std::collections::HashMap;

#[derive(new)]
pub struct EventProcessing {

}

impl EventProcessing {
    pub async fn emit_event(&self, event: PackedEvent) -> Result<(), Failure> 
    {
        unimplemented!()
    }
}