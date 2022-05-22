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

use std::string::ToString;
use strum_macros::Display;


#[derive(Display)]
enum MenuItem {
    RecordTask,
    NewTask,
    DeleteTask
}

struct MenuList {
    state: ListState,
    items: Vec<MenuItem>
}


impl MenuList {
    fn new() -> Self {
        MenuList {
            state: ListState::default(),
            items: vec![MenuItem::RecordTask, MenuItem::NewTask, MenuItem::DeleteTask]
        }
    }
}


pub struct Menu {
    menu_list: MenuList
}

impl<'a> Menu {
    // So we can pass this into `ui()` func w/o creating widget impl for now
    pub fn new() -> List<'a> {

        let menu = Menu {
            menu_list: MenuList::new()
        };

        let items: Vec<ListItem> = menu.menu_list.items.iter().map(|item| {
            let thing: String = item.to_string();
            let text = Text::from(thing);
            ListItem::new(text)
        }).collect();

        List::new(items).highlight_style(
            Style::default()
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD)
        )
    }
}

