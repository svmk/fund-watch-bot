#[derive(Debug)]
pub struct EdgarFragment {
    text: String,
}

impl EdgarFragment {
    pub fn new() -> EdgarFragment {
        return EdgarFragment {
            text: String::new(),
        }
    }

    pub fn push_line(&mut self, line: &str) {
        self.text.push_str(line);
        self.text.push_str("\n");
    }

    pub fn get_text(&self) -> &str {
        return &self.text;
    }
}