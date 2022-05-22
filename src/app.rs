

use tui::widgets::ListState;
use crate::menu::*;


enum AppState {
    // TODO: fill in with Menu, etc. for processing events
    // and creating a focus system
}

/// Store the current state of the running appplicarion
pub struct App {
    pub menu: Menu
}


impl App {
    pub fn new() -> Self {
        App {
            menu: Menu::new()
        }
    }


    // MARK: Menu interface
    // This will need a refactor once states are implemented.
    pub fn menu_prev(&mut self) {
        self.menu.select_prev();
    }

    pub fn menu_next(&mut self) {
        self.menu.select_next();
    }

    pub fn menu_select(&self) {
        if let Some(menu_item) = self.menu.select() {
            match menu_item {
                MenuItem::RecordTask => println!("RecordTask selected"),
                MenuItem::NewTask    => println!("NewTask selected"),
                MenuItem::DeleteTask => println!("DeleteTask selected"),
            }
        }
    }
}
