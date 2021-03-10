use crate::prelude::*;
use crate::event_emitter::model::packed_event::PackedEvent;
use crate::event_emitter::service_handler::packed_event_handler::PackedEventHandler;
use std::sync::Arc;

pub struct EventExecutor {
    event_handler: Box<dyn PackedEventHandler>,
    reference_count: Arc<()>,
}

impl EventExecutor {
    pub fn new(event_handler: Box<dyn PackedEventHandler>) -> (EventExecutor, EventExecutorHandle) {
        let reference_count = Arc::new(());
        let event_executor = EventExecutor {
            event_handler,
            reference_count: reference_count.clone(),
        };
        let event_executor_handle = EventExecutorHandle {
            reference_count,
        };
        return (event_executor, event_executor_handle);
    }

    pub async fn execute(&self, event: PackedEvent) -> Result<(), Failure> {
        self.event_handler.handle_event(event).await?;
        return Ok(());
    }

    pub fn is_closed(&self) -> bool {
        return Arc::strong_count(&self.reference_count) <= 1;
    }
}

#[derive(Debug)]
pub struct EventExecutorHandle {
    reference_count: Arc<()>,
}