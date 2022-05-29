
use tui::widgets::*;
use tui::layout::*;


pub struct InputManager<'a> {
    pub input_text: String,
    pub cursor: &'a str,
    cur_task_name: Option<String>
}


impl<'a> InputManager<'a> {
    pub fn new() -> Self {
        InputManager {
            input_text: String::from(""),
            cursor: "_",
            cur_task_name: None
        }
    }

    /// A dummy method so we can say the current task name
    /// without hooking into the data model
    pub fn note_task(&mut self, cur_task_name: String) {
        self.cur_task_name = Some(cur_task_name);
    }


    // MARK: Event handling

    pub fn keypress(&mut self, key: char) {
        self.input_text.push(key);
    }

    pub fn backspace(&mut self) {
        self.input_text.pop();
    }

    pub fn clear_input(&mut self) {
        self.input_text.clear();
    }

    pub fn make_input_task_name(&mut self) -> Paragraph {
        let text = String::from("input: ") + self.input_text.as_str() + self.cursor;
        Paragraph::new(text.clone())
            .alignment(Alignment::Left)
            .block(Block::default().title("Create new task").borders(Borders::all()))
    }

    pub fn make_input_task_desc(&mut self) -> Paragraph {

        if self.cur_task_name.is_none() {
            panic!("No previous task available.");
        }

        let text1 = String::from("input: ".to_owned() + self.cur_task_name.as_ref().unwrap().as_str());
        let text2 = String::from("desc:  ".to_owned() + self.input_text.as_str() + self.cursor);
        let text = format!("{}\n{}", text1, text2);

        Paragraph::new(text.clone())
            .alignment(Alignment::Left)
            .block(Block::default().title("Create new task").borders(Borders::all()))
    }
}
