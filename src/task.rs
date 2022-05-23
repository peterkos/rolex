

use chrono::prelude::*;
use uuid::Uuid;

use std::fmt::Display;

pub enum EventState {
    Active,
    Inactive
    // TODO: Add more states; deferred?
}

pub struct Task<'a> {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub name: &'a str,
    pub desc: Option<&'a str>,
    pub uuid: Uuid,
    pub state: EventState,
}

impl<'a> Task<'a> {

    fn new(name: &'a str, desc: Option<&'a str>) -> Self {
        // Note that events are inactive by default
        Task {
            start: None,
            end: None,
            name,
            desc,
            uuid: Uuid::new_v4(),
            state: EventState::Inactive
        }
    }
}

impl<'a> Display for Task<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "name: {:?}, desc: {:?}", self.name, self.desc)
    }
}




