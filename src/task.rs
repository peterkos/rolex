

use chrono::prelude::*;
use uuid::Uuid;

use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TaskState {
    Active,
    Inactive
    // TODO: Add more states; deferred?
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Task<'a> {
    pub created: DateTime<Utc>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub name: String,
    pub desc: Option<&'a str>,
    pub uuid: Uuid,
    pub state: TaskState,
}


impl<'a> Task<'a> {

    pub fn new(name: String, desc: Option<&'a str>) -> Self {
        // Note that events are inactive by default,
        // as tasks are not started
        // until the user manually begins tracking that task.
        Task {
            created: Utc::now(),
            start: None,
            end: None,
            name: name.clone(),
            desc,
            uuid: Uuid::new_v4(),
            state: TaskState::Inactive
        }
    }
}

impl<'a> Display for Task<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "name: {:?}, desc: {:?}", self.name, self.desc)
    }
}




