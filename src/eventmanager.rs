


use std::collections::HashMap;
use chrono::prelude::*;
use uuid::Uuid;

use crate::{TrackEvent, EventState};



struct EventManager<'a> {
    events: HashMap<Uuid, TrackEvent<'a>>
}

impl<'a> EventManager<'a> {

    fn add(&mut self, event: TrackEvent<'a>) {
        self.events.insert(event.uuid, event);
    }

    fn start(&mut self, uuid: Uuid) {
        // Grumble grumble, let...else is only in nightly :(
        if let Some(event) = self.events.get_mut(&uuid) {
            event.start = Some(Utc::now());
            event.state = EventState::Active;
        }
    }

    fn end(&mut self, uuid: Uuid) {
        if let Some(event) = self.events.get_mut(&uuid) {
            event.end = Some(Utc::now());
            event.state = EventState::Inactive;
        }
    }


}
