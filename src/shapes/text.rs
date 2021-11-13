use crate::consts::CHAR_NEWLINE;

use super::{Shape, ShapeRender};

#[derive(Debug)]
pub struct TextShape {
    position: (usize, usize),
    content: String,
    preview: bool,
}

impl ShapeRender for TextShape {
    fn draw(
        &mut self,
        grid_buffer: &mut Vec<crate::data::GridCell>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        let (rows, cols) = grid;
        let (mut row, mut col) = self.position;
        for c in self.content.chars() {
            if c != CHAR_NEWLINE {
                let i = row * cols + col;
                grid_buffer[i].set_content(c);
                col += 1;
            } else {
                row += 1;
                col = self.position.1;
            }
        }
    }

    fn commit(&mut self, grid_buffer: &mut Vec<crate::data::GridCell>) {}

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
        }
    }
}
