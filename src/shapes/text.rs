use super::{Shape, ShapeRender};

#[derive(Debug)]
pub struct TextShape {
    position: (usize, usize),
    content: String,
    preview: bool,
    max_len: usize,
}

impl ShapeRender for TextShape {
    fn draw(&mut self, grid_buffer: &mut crate::data::GridList) {
        let (_rows, cols) = grid_buffer.grid_size;
        let (row, col) = self.position;

        for index in 0..=self.max_len {
            let i = row * cols + col + index;
            if let Some(c) = self.content.chars().nth(index) {
                grid_buffer.get(i).set_content(c);
            } else {
                grid_buffer.get(i).set_content(' ');
            }
        }
    }

    fn commit(&mut self, _grid_buffer: &mut crate::data::GridList) {
        self.preview = false;
    }

    fn is_preview(&self) -> bool {
        self.preview
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_manual_commit(&self) -> bool {
        true
    }
}

impl Shape for TextShape {}

impl TextShape {
    pub fn new(row: usize, col: usize, text: &str) -> Self {
        Self {
            position: (row, col),
            content: text.to_string(),
            preview: true,
            max_len: 0,
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.content.push(c);
        self.max_len = self.max_len.max(self.content.len());
    }

    pub fn pop_char(&mut self) {
        self.content.pop();
    }
}
