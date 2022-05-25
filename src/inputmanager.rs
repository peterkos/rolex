
use tui::widgets::*;
use tui::layout::*;


pub struct InputManager<'a> {
    pub input_text: &'a str,
    pub cursor: &'a str
}


impl<'a> InputManager<'a> {


    pub fn new() -> Self {
        InputManager {
            input_text: "input: ",
            cursor: ""
        }
    }

    pub fn cancel_input(&mut self) {
        self.input_text = "";
        self.cursor = "";
    }


    pub fn make_input(&mut self) -> Paragraph {
        Paragraph::new(self.input_text.clone())
            .alignment(Alignment::Left)
    }
}
