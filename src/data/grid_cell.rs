use crate::consts::CHAR_SPACE;
use crate::data::overlap;
use crate::shapes::line::LineDirection;

#[derive(Clone, Copy)]
pub struct GridCell {
    pub highlight_index: usize,
    pub content: char,
    pub preview: Option<char>,
    pub highlighted: bool,
    pub line_direction: Option<LineDirection>,
}

impl GridCell {
    pub fn new(content: char) -> Self {
        Self {
            highlight_index: 0,
            content,
            preview: None,
            highlighted: false,
            line_direction: None,
        }
    }

    pub fn set_line_direction(&mut self, direction: LineDirection) {
        self.line_direction = Some(direction);
    }

    pub fn empty() -> Self {
        GridCell::new(CHAR_SPACE)
    }

    pub fn read(&self) -> (char, char) {
        let content = self.content;
        let preview = self.preview.unwrap_or(' ');
        (content, preview)
    }

    pub fn read_content(&self) -> char {
        self.content
    }

    pub fn clear(&mut self) {
        self.content = CHAR_SPACE;
        self.preview = None;
        self.highlight_index = 0;
    }

    pub fn set_content(&mut self, content: char) {
        self.content = content;
    }

    pub fn set_preview(&mut self, content: char) {
        self.preview = Some(content);
    }

    pub fn commit(&mut self) {
        if let Some(preview) = self.preview {
            // TODO: Implement line overlap processing here
            // Each cell should carry an information about the starting point and the drawing
            // direction, so the overlap algorithm could use
            self.content =
                overlap::calculate_cell_content(self.line_direction, self.content, preview);
            self.preview = None;
            self.line_direction = None;
        }
    }

    pub fn discard(&mut self) {
        self.preview = None;
        self.line_direction = None;
    }

    pub fn highlight(&mut self, highlight_index: usize) {
        self.highlighted = true;
        self.highlight_index = highlight_index;
    }

    pub fn clear_highlight(&mut self) {
        self.highlighted = false;
        self.highlight_index = 0;
    }
}
