
#![allow(unused)]


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, os::unix::net, rc::Rc, borrow::BorrowMut, cell::RefCell};
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

    let view_model = Rc::new(RefCell::new(ViewModel::new()));
    let res = run_app(&mut terminal, view_model);

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
fn run_app<B: Backend>(terminal: &mut Terminal<B>, view_model: Rc<RefCell<ViewModel>>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, Rc::clone(&view_model)))?;


        if let Event::Key(key) = event::read()? {

            // I believe the technical term here is "oof"
            // Manually quit using Ctrl+c
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                return Ok(())
            }

            if view_model.borrow().state == AppState::Typing {

                // Special code to quit typing is Esc
                if let KeyCode::Esc = key.code {
                    RefCell::borrow_mut(&view_model).cancel_input();
                }

                // If Enter is pressed we want to treat that as good input
                if let KeyCode::Enter = key.code {
                    RefCell::borrow_mut(&view_model).create_task();
                }

                // Otherwise, we want to forward our keypress into the InputManager
                if let KeyCode::Char(chr) = key.code {
                    RefCell::borrow_mut(&view_model).input_manager.keypress(chr);
                }

                // Manual deletion...
                if let KeyCode::Backspace = key.code {
                    RefCell::borrow_mut(&view_model).input_manager.backspace();
                }

                // TODO: Implement arrow movement
                // TODO: Implement option+left/right for per-word traversal

                continue
            }

            // Global: quit on `q`
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            // Global: show menu on `m`
            if let KeyCode::Char('m') = key.code {
                RefCell::borrow_mut(&view_model).state = AppState::Menu;
            }

            // App state is manged by view model,
            // so we leave it up to the view model
            // to decide which view should receive
            // the key commands.
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right | KeyCode::Down => {
                    RefCell::borrow_mut(&view_model).list_operation(ManagedListState::Prev)
                },
                KeyCode::Left | KeyCode::Up => {
                    RefCell::borrow_mut(&view_model).list_operation(ManagedListState::Next)
                },
                KeyCode::Enter => {
                    RefCell::borrow_mut(&view_model).list_operation(ManagedListState::Select)
                }
                default => ()
            };
        }
    }
}


fn ui<B: Backend>(f: &mut Frame<B>, view_model_ref: Rc<RefCell<ViewModel>>) {


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

    // I really don't like this pattern,
    // but it might be the only option for this UI style.

    let mut view_model = RefCell::borrow_mut(&view_model_ref);

    // FIXME: For now, always put task list on right half
    f.render_stateful_widget(view_model.task_manager.make_list(), main[1], &mut view_model.task_manager.task_list.state);

    match view_model.state {
        AppState::Menu => {
            // Menu on left half
            f.render_stateful_widget(view_model.menu_manager.make_list(), main[0], &mut view_model.menu_manager.menu_list.state);

        },
        AppState::NewTask | AppState::Typing => {
            // Task list on left half
            // f.render_stateful_widget(view_model.task_manager.make_newtask(), main[0], &mut view_model.task_manager.task_list.state);

            f.render_widget(view_model.input_manager.make_input(), main[0]);

            // Now that NewTask has loaded,
            // we want the user to automatically start typing.
            view_model.state = AppState::Typing;
        },
        AppState::RecordTask => {
            // Task list on left half
            f.render_stateful_widget(view_model.task_manager.make_list(), main[0], &mut view_model.task_manager.task_list.state);
        },
        AppState::DeleteTask => todo!(),
        // AppState::Typing => () // FIXME: Ignore typing state for now
    }

    // Draw menu help text
    // This needs to be done **after** the views are rendered
    // because the rendering of some views triggers modes to be activated in that view
    // (i.e., Typing).
    let text = match view_model.state {
        AppState::Typing => "Press Esc to quit",
        _                => "Press q to quit, m for menu"
    };
    let paragraph = Paragraph::new(text.clone())
        .block(Block::default())
        .alignment(Alignment::Center);
    f.render_widget(paragraph, wrapper[2]);


}


