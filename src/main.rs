
#![allow(unused)]


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};


mod viewmodel;
use viewmodel::*;

mod menu;
use menu::*;

mod task;
use task::*;

mod taskmanager;
use taskmanager::*;


fn main() -> Result<(), Box<dyn Error>> {

    // Instatiate our menus now


    // Mostly copied from the blocks example
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = ViewModel::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}


// TODO: Factor out generic
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut view_model: ViewModel) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut view_model))?;

        // Quit when press q
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            // App state is manged by view model,
            // so we leave it up to the view model
            // to decide which view should receive
            // the key commands.
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right | KeyCode::Down => {
                    view_model.list_operation(ManagedListState::Prev)
                },
                KeyCode::Left | KeyCode::Up => {
                    view_model.list_operation(ManagedListState::Next)
                },
                KeyCode::Enter => {
                    view_model.list_operation(ManagedListState::Select)
                }
                default => ()
            };
        }
    }
}



fn ui<B: Backend>(f: &mut Frame<B>, view_model: &mut ViewModel) {

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // I really don't like this pattern,
    // but it might be the only option for this UI style.
    match view_model.state {
        AppState::Menu => {
            // Menu on left half
            f.render_stateful_widget(view_model.menu_manager.make_list(), chunks[0], &mut view_model.menu_manager.menu_list.state);

        // Random block on right half for now
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        },
        AppState::NewTask => {

            // Task list on left half
            f.render_stateful_widget(view_model.task_manager.make_list(), chunks[0], &mut view_model.task_manager.task_list.state);

            // Random block on right half for now
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        },
        AppState::RecordTask => todo!(),
        AppState::DeleteTask => todo!(),
    }


}


