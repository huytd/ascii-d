use crate::consts::CHAR_NEWLINE;

use super::{Shape, ShapeRender};

#[derive(Debug)]
pub struct TextShape {
    position: (usize, usize),
    content: String,
    preview: bool,
    max_len: usize,
}

impl ShapeRender for TextShape {
    fn draw(
        &mut self,
        grid_buffer: &mut crate::data::GridList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        let (rows, cols) = grid;
        let (mut row, mut col) = self.position;

        for (index, c) in self.content.chars().enumerate() {
            let i = row * cols + col;
            if c != CHAR_NEWLINE {
                grid_buffer.get(i).set_content(c);
                col += 1;
            } else {
                row += 1;
                col = self.position.1;
            }

            if self.is_preview() && index == self.content.len() - 1 {
                grid_buffer.highlight(i);
            }
        }
    }

    fn commit(&mut self, grid_buffer: &mut crate::data::GridList) {
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
