

use tui::widgets::ListState;
use crate::menu::*;


/// All the possible views that can be rendered on screen
/// (i.e., all different views we need to make & switch between...)
pub enum AppState {
    Menu,
    NewTask,
    RecordTask,
    DeleteTask
}

/// Store the current state of the running appplicarion
pub struct ViewModel {
    pub menu: Menu,
    pub state: AppState
}


impl ViewModel {
    pub fn new() -> Self {
        ViewModel {
            menu: Menu::new(),
            state: AppState::Menu
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
                MenuItem::RecordTask => (),
                MenuItem::NewTask    => {

                },
                MenuItem::DeleteTask => (),
            }
        }
    }
}
