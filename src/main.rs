
#![allow(unused)]


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, os::unix::net};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
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

mod inputmanager;
use inputmanager::*;


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


        if let Event::Key(key) = event::read()? {

            // I believe the technical term here is "oof"
            // Manually quit using Ctrl+c
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                return Ok(())
            }

            if view_model.state == AppState::Typing {
                continue
            }

            if view_model.state == AppState::Typing {

                // Special code to quit typing is Esc
                if let KeyCode::Esc = key.code {
                    view_model.cancel_input();
                }
            }

            // Global: quit on `q`
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            // Global: show menu on `m`
            if let KeyCode::Char('m') = key.code {
                view_model.state = AppState::Menu;
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


    /*
        |-------|
        |-------|   wrapper[0]

        |---|---|
        |   |   |   wrapper[1]
        |---|---|
          ^   ^---- main[1]
          |-------- main[0]

        |-------|   wrapper[2]
        |-------|

    */

    // Create main wrapper around views
    let wrapper = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
        .vertical_margin(6)
        .horizontal_margin(30)
        .split(f.size());

    // Inner layout used for main split panes
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(wrapper[1]);

    let newtask_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(2)].as_ref())
        .split(main[0]);

    // Draw menu help text
    let text = "Press q to quit, m for menu";
    let paragraph = Paragraph::new(text.clone())
        .block(Block::default())
        .alignment(Alignment::Center);
    f.render_widget(paragraph, wrapper[2]);

    // I really don't like this pattern,
    // but it might be the only option for this UI style.
    match view_model.state {
        AppState::Menu => {
            // Menu on left half
            f.render_stateful_widget(view_model.menu_manager.make_list(), main[0], &mut view_model.menu_manager.menu_list.state);

            // Random block on right half for now
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, main[1]);
        },
        AppState::NewTask => {
            // Task list on left half
            // f.render_stateful_widget(view_model.task_manager.make_newtask(), main[0], &mut view_model.task_manager.task_list.state);

            f.render_widget(view_model.input_manager.make_input(), newtask_layout[0]);

            // Now that NewTask has loaded,
            // we want the user to automatically start typing.
            view_model.state = AppState::Typing;

            // Random block on right half for now
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, main[1]);
        },
        AppState::RecordTask => {
            // Task list on left half
            f.render_stateful_widget(view_model.task_manager.make_list(), main[0], &mut view_model.task_manager.task_list.state);

            // Random block on right half for now
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, main[1]);
        },
        AppState::DeleteTask => todo!(),
        AppState::Typing => () // FIXME: Ignore typing state for now
    }


}


