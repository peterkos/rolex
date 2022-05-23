
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


mod app;
use app::*;

mod menu;
use menu::*;

mod trackevent;
use trackevent::*;

mod eventmanager;
use eventmanager::*;


fn main() -> Result<(), Box<dyn Error>> {

    // Instatiate our menus now


    // Mostly copied from the blocks example
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create our App instance
    let app = App::new();
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
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // Quit when press q
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right | KeyCode::Down => {
                    app.menu_prev()
                },
                KeyCode::Left | KeyCode::Up => {
                    app.menu_next()
                },
                KeyCode::Enter => {
                    app.menu_select()
                }
                default => ()
            };
        }
    }
}



fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {


    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());


    // Menu on left half
    f.render_stateful_widget(app.menu.make_list(), chunks[0], &mut app.menu.menu_list.state);

    // Random block on right half for now
    let block = Block::default().title("With borders").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}


