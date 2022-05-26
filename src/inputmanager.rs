
use tui::widgets::*;
use tui::layout::*;


pub struct InputManager<'a> {
    pub input_text: String,
    pub cursor: &'a str
}


impl<'a> InputManager<'a> {


    pub fn new() -> Self {
        InputManager {
            input_text: String::from(""),
            cursor: "_"
        }
    }

    pub fn keypress(&mut self, key: char) {
        self.input_text.push(key);
    }

    pub fn backspace(&mut self) {
        self.input_text.pop();
    }

    pub fn clear_input(&mut self) {
        self.input_text.clear();
    }

    pub fn make_input(&mut self) -> Paragraph {
        let text = String::from("input: ") + self.input_text.as_str() + self.cursor;
        Paragraph::new(text.clone())
            .alignment(Alignment::Left)
            .block(Block::default().title("Create new task").borders(Borders::all()))
    }
}
