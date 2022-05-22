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
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;


#[derive(Display, EnumIter)]
pub enum MenuItem {
    RecordTask,
    NewTask,
    DeleteTask
}

pub struct MenuList {
    pub state: ListState,
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
    pub menu_list: MenuList
}

impl<'a> Menu {
    // So we can pass this into `ui()` func w/o creating widget impl for now
    pub fn new() -> Self {
        Menu {
            menu_list: MenuList::new()
        }

    }

    pub fn make_list(&self) -> List<'a> {

        let items: Vec<ListItem> = self.menu_list.items.iter().map(|item| {
            let thing: String = item.to_string();
            let text = Text::from(thing);
            ListItem::new(text)
        }).collect();

        List::new(items)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
                )
                .highlight_symbol(">> ")
    }

    pub fn select(&self) -> Option<MenuItem> {
        if let Some(i) = self.menu_list.state.selected() {
            return MenuItem::iter().nth(i)
        }
        None
    }

    pub fn select_prev(&mut self) {
        let i = match self.menu_list.state.selected() {
            Some (i) => {
                if i > 2 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0
        };
        self.menu_list.state.select(Some(i));
    }

    pub fn select_next(&mut self) {
        let i = match self.menu_list.state.selected() {
            Some (i) => {
                if i == 0 {
                    2
                } else {
                    i - 1
                }
            }
            None => 0
        };
        self.menu_list.state.select(Some(i));
    }
}

