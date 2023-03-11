use druid::Point;

use crate::{
    consts::{
        CHAR_ARROW_DOWN, CHAR_ARROW_LEFT, CHAR_ARROW_RIGHT, CHAR_ARROW_UP, CHAR_HOR_L, CHAR_VER_L,
    },
    data::grid_list::GridList,
};

use super::ShapeRender;

pub enum LineDirection {
    Horizontal,
    Vertical,
}

pub struct LineShape {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub direction: LineDirection,
    pub preview: bool,
}

impl_shape_for!(LineShape);

impl ShapeRender for LineShape {
    fn draw(&mut self, grid_buffer: &mut GridList) {
        let (_rows, cols) = grid_buffer.grid_size;
        let (from_row, from_col) = self.start;
        let (to_row, to_col) = self.end;

        grid_buffer.discard_all();

        let start_i = from_row * cols + from_col;

        match self.direction {
            LineDirection::Vertical => {
                let from = if from_row > to_row { to_row } else { from_row };
                let to = if from_row > to_row { from_row } else { to_row };

                for row in from..=to {
                    let i = row * cols + from_col;
                    grid_buffer.get(i).set_preview(CHAR_VER_L);
                }

                if from_row > to_row {
                    let head_i = from * cols + from_col;
                    grid_buffer.get(head_i).set_preview(CHAR_ARROW_DOWN);
                } else {
                    let head_i = to * cols + from_col;
                    grid_buffer.get(head_i).set_preview(CHAR_ARROW_UP);
                }

                // Mark start of the line
                grid_buffer.get(start_i).set_preview('-');
            }
            LineDirection::Horizontal => {
                let from = if from_col > to_col { to_col } else { from_col };
                let to = if from_col > to_col { from_col } else { to_col };
                for col in from..=to {
                    let i = from_row * cols + col;
                    grid_buffer.get(i).set_preview(CHAR_HOR_L);
                }

                if from_col > to_col {
                    let head_i = from_row * cols + from;
                    grid_buffer.get(head_i).set_preview(CHAR_ARROW_LEFT);
                } else {
                    let head_i = from_row * cols + to;
                    grid_buffer.get(head_i).set_preview(CHAR_ARROW_RIGHT);
                }

                // Mark start of the line
                grid_buffer.get(start_i).set_preview('.');
            }
        }
    }

    fn commit(&mut self, grid_buffer: &mut GridList) {
        grid_buffer.commit_all();
        self.preview = false;
    }

    fn is_preview(&self) -> bool {
        self.preview
    }

    fn is_manual_commit(&self) -> bool {
        false
    }
}

impl LineShape {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            start: (row, col),
            end: (row, col),
            direction: LineDirection::Horizontal,
            preview: true,
        }
    }
}
