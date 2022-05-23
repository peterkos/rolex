

use tui::widgets::ListState;
use crate::*;


/// All the possible views that can be rendered on screen
/// (i.e., all different views we need to make & switch between...)
pub enum AppState {
    Menu,
    NewTask,
    RecordTask,
    DeleteTask
}

pub enum ManagedListState {
    Prev,
    Next,
    Select
}

/// Store the current state of the running appplicarion
pub struct ViewModel<'a> {
    pub menu_manager: MenuManager,
    pub task_manager: TaskManager<'a>,
    pub state: AppState,
}


impl<'a> ViewModel<'a> {
    pub fn new() -> Self {
        ViewModel {
            menu_manager: MenuManager::new(),
            task_manager: TaskManager::new(),
            state: AppState::Menu,
        }
    }

    pub fn list_operation(&mut self, state: ManagedListState) {
        match self.state {
            AppState::Menu       => self.menu_manager.list_operation(state),
            AppState::NewTask    => todo!(),
            AppState::RecordTask => todo!(),
            AppState::DeleteTask => todo!(),
        }

    }

    // MARK: Menu interface

    // This will need a refactor once states are implemented.
    pub fn menu_prev(&mut self) {
        self.menu_manager.select_prev();
    }

    pub fn menu_next(&mut self) {
        self.menu_manager.select_next();
    }

    pub fn menu_select(&self) {
        if let Some(menu_item) = self.menu_manager.select() {
            match menu_item {
                MenuItem::RecordTask => (),
                MenuItem::NewTask    => {

                },
                MenuItem::DeleteTask => (),
            }
        }
    }

    // MARK: Task List interface
    pub fn

}
