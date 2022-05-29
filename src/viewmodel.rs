

use tui::widgets::ListState;
use crate::*;


/// All the possible views that can be rendered on screen
/// (i.e., all different views we need to make & switch between...)
#[derive(PartialEq, Eq, Debug)]
pub enum AppState {
    Menu,
    RecordTask,
    DeleteTask,
    Typing, // Special state for the event handler to ignore input
    NewTaskName,
    NewTaskDesc, // FIXME: Need per-manager states...
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
    pub input_manager: InputManager<'a>,
    pub state: AppState,
}


impl<'a> ViewModel<'a> {
    pub fn new() -> Self {
        ViewModel {
            menu_manager: MenuManager::new(),
            task_manager: TaskManager::new(),
            input_manager: InputManager::new(),
            state: AppState::Menu,
        }
    }

    pub fn list_operation(&mut self, state: ManagedListState) {
        match self.state {
            AppState::Menu => {
                let selected = self.menu_manager.list_operation(state);

                if let Some(selected) = selected {
                    match selected {
                        MenuItem::RecordTask => self.state = AppState::RecordTask,
                        MenuItem::NewTask    => self.state = AppState::NewTaskName,
                        MenuItem::DeleteTask => self.state = AppState::DeleteTask,
                    }
                }

            },
            AppState::NewTaskName    => todo!(),
            AppState::RecordTask => todo!(),
            AppState::DeleteTask => todo!(),
            AppState::Typing     => todo!(),
            AppState::NewTaskDesc => todo!()
        }
    }

    // MARK: Input Handling

    pub fn create_task(&mut self) {
        let name = self.input_manager.input_text.clone();
        self.task_manager.create_task(name.clone(), None);

        // This is so the input can fake a "prompt" history
        // with the preivous (valid) input of the user
        self.input_manager.note_task(name.clone());

        // Set app state for next phase of input
        self.state = AppState::NewTaskDesc;

        // Cleanup
        self.input_manager.clear_input();
    }

    pub fn cancel_input(&mut self) {
        self.input_manager.clear_input();
        self.state = AppState::Menu;
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
    // pub fn

}
