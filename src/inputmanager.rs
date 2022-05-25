
use tui::widgets::*;
use tui::layout::*;


pub struct InputManager<'a> {
    pub input_text: String,
    pub cursor: &'a str
}


impl<'a> InputManager<'a> {


    pub fn new() -> Self {
        InputManager {
            input_text: String::from("input: "),
            cursor: "_"
        }
    }

    pub fn keypress(&mut self, key: char) {
        self.input_text.push(key);
    }

    pub fn cancel_input(&mut self) {
        self.input_text.clear();
        self.cursor = "";
    }

    pub fn make_input(&mut self) -> Paragraph {
        let text = self.input_text.to_owned() + self.cursor;
        Paragraph::new(text.clone())
            .alignment(Alignment::Left)
    }
}
