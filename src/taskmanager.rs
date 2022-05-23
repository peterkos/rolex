


use std::collections::HashMap;
use chrono::prelude::*;
use uuid::Uuid;

use crate::{Task, EventState};


pub struct TaskManager<'a> {
    events: HashMap<Uuid, Task<'a>>
}

impl<'a> TaskManager<'a> {

    pub fn new() -> Self {
        TaskManager { events: HashMap::new() }
    }

    pub fn add(&mut self, event: Task<'a>) {
        self.events.insert(event.uuid, event);
    }

    pub fn start(&mut self, uuid: Uuid) {
        // Grumble grumble, let...else is only in nightly :(
        if let Some(event) = self.events.get_mut(&uuid) {
            event.start = Some(Utc::now());
            event.state = EventState::Active;
        }
    }

    pub fn end(&mut self, uuid: Uuid) {
        if let Some(event) = self.events.get_mut(&uuid) {
            event.end = Some(Utc::now());
            event.state = EventState::Inactive;
        }
    }


}
