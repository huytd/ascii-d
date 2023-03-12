use druid::Point;

use crate::{
    consts::{
        CHAR_ARROW_DOWN, CHAR_ARROW_LEFT, CHAR_ARROW_RIGHT, CHAR_ARROW_UP, CHAR_HOR_L, CHAR_VER_L,
    },
    data::grid_list::GridList,
};

use super::ShapeRender;

#[derive(Clone, Copy)]
pub enum LineDirection {
    RightToLeft,
    LeftToRight,
    UpToDown,
    DownToUp
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
        grid_buffer.get(start_i).set_line_direction(self.direction);

        match self.direction {
            LineDirection::UpToDown => {
                for row in from_row..=to_row {
                    let i = row * cols + from_col;
                    grid_buffer.get(i).set_preview(CHAR_VER_L);
                }
                let head_i = to_row * cols + from_col;
                grid_buffer.get(head_i).set_preview(CHAR_ARROW_UP);
            },
            LineDirection::DownToUp => {
                for row in to_row..=from_row {
                    let i = row * cols + from_col;
                    grid_buffer.get(i).set_preview(CHAR_VER_L);
                }
                let head_i = to_row * cols + from_col;
                grid_buffer.get(head_i).set_preview(CHAR_ARROW_DOWN);
            },
            LineDirection::RightToLeft => {
                for col in from_col..=to_col {
                    let i = from_row * cols + col;
                    grid_buffer.get(i).set_preview(CHAR_HOR_L);
                }
                let head_i = from_row * cols + to_col;
                grid_buffer.get(head_i).set_preview(CHAR_ARROW_RIGHT);
            },
            LineDirection::LeftToRight => {
                for col in to_col..=from_col {
                    let i = from_row * cols + col;
                    grid_buffer.get(i).set_preview(CHAR_HOR_L);
                }
                let head_i = from_row * cols + to_col;
                grid_buffer.get(head_i).set_preview(CHAR_ARROW_LEFT);
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
            direction: LineDirection::RightToLeft,
            preview: true,
        }
    }
}
