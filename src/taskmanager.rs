use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};


use std::collections::HashMap;
use chrono::prelude::*;
use uuid::Uuid;
use std::string::ToString;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;




use crate::{Task, EventState};


pub struct TaskList {
    pub state: ListState,
}


impl TaskList {
    pub fn new() -> Self {
        TaskList {
            state: ListState::default()
        }
    }

}

pub struct TaskManager<'a> {
    events: HashMap<Uuid, Task<'a>>,
    pub task_list: TaskList
}

impl<'a> TaskManager<'a> {

    pub fn new() -> Self {
        TaskManager {
            events: HashMap::new(),
            task_list: TaskList::new()
        }
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

    // MARK: [View] Make New Task
    pub fn make_newtask(&mut self) -> Block<'a> {
        Block::default().title("New Task").borders(Borders::all())
    }

    // MARK: [View] Make List

    pub fn make_list(&self) -> List<'a> {
        let items: Vec<ListItem> = self.events.iter().map(|item| {
            let thing: String = item.1.to_string();
            let text = Text::from(thing);
            ListItem::new(text)
        }).collect();

        List::new(items)
            .block(Block::default().title("Menu").borders(Borders::all()))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
                )
                .highlight_symbol(">> ")
    }

}
