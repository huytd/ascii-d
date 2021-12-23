use crate::consts::{
    CHAR_CORNER_BL_L, CHAR_CORNER_BR_L, CHAR_CORNER_TL_L, CHAR_CORNER_TR_L, CHAR_HOR_L, CHAR_VER_L,
};

use super::{Shape, ShapeRender};

#[derive(Debug)]
pub struct RectShape {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub preview: bool,
}

impl RectShape {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            start: (row, col),
            end: (row, col),
            preview: true,
        }
    }
}

impl Shape for RectShape {}

impl ShapeRender for RectShape {
    fn draw(&mut self, grid_buffer: &mut crate::data::GridList) {
        let (_rows, cols) = grid_buffer.grid_size;
        let (f_row, f_col) = self.start;
        let (t_row, t_col) = self.end;

        let from_row = f_row.min(t_row);
        let from_col = f_col.min(t_col);
        let to_row = f_row.max(t_row);
        let to_col = f_col.max(t_col);

        grid_buffer.discard_all();

        for col in from_col..=to_col {
            let i = from_row * cols + col;
            grid_buffer.get(i).set_preview(CHAR_HOR_L);
            let i2 = to_row * cols + col;
            grid_buffer.get(i2).set_preview(CHAR_HOR_L);
        }

        for row in from_row..=to_row {
            let i = row * cols + from_col;
            grid_buffer.get(i).set_preview(CHAR_VER_L);
            let i2 = row * cols + to_col;
            grid_buffer.get(i2).set_preview(CHAR_VER_L);
        }

        let top_left = from_row * cols + from_col;
        grid_buffer.get(top_left).set_preview(CHAR_CORNER_TL_L);

        let top_right = from_row * cols + to_col;
        grid_buffer.get(top_right).set_preview(CHAR_CORNER_TR_L);

        let bottom_right = to_row * cols + to_col;
        grid_buffer.get(bottom_right).set_preview(CHAR_CORNER_BR_L);

        let bottom_left = to_row * cols + from_col;
        grid_buffer.get(bottom_left).set_preview(CHAR_CORNER_BL_L);
    }

    fn commit(&mut self, grid_buffer: &mut crate::data::GridList) {
        grid_buffer.commit_all();
        self.preview = false;
    }

    fn is_preview(&self) -> bool {
        self.preview
    }

    fn is_manual_commit(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
