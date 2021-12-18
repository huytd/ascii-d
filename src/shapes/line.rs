use crate::consts::{
    CHAR_CORNER_BL_L, CHAR_CORNER_BR_L, CHAR_CORNER_TL_L, CHAR_CORNER_TR_L, CHAR_HOR_DOWN_L,
    CHAR_HOR_L, CHAR_HOR_UP_L, CHAR_SPACE, CHAR_VER_L, CHAR_VER_LEFT_L, CHAR_VER_RIGHT_L,
};

use super::{Shape, ShapeRender};

#[derive(Debug)]
pub enum LineDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct LineShape {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub direction: LineDirection,
    pub preview: bool,
}

impl ShapeRender for LineShape {
    fn draw(&mut self, grid_buffer: &mut crate::data::GridList) {
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

                if grid_buffer.get(start_i).read_content() == CHAR_HOR_L {
                    let prev_i = from_row * cols + (from_col - 1);
                    let next_i = from_row * cols + (from_col + 1);
                    if grid_buffer.get(prev_i).read() == CHAR_SPACE {
                        if from_row < to_row {
                            // Draw down, put top-left corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_TL_L);
                        }
                        if from_row > to_row {
                            // Draw up, put bottom-left corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_BL_L);
                        }
                    } else if grid_buffer.get(next_i).read() == CHAR_SPACE {
                        if from_row < to_row {
                            // Draw down, put top-right corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_TR_L);
                        }
                        if from_row > to_row {
                            // Draw up, put bottom-right corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_BR_L);
                        }
                    } else {
                        if from_row < to_row {
                            // Draw down, put hor-down
                            grid_buffer.get(start_i).set_preview(CHAR_HOR_DOWN_L);
                        }
                        if from_row > to_row {
                            // Draw up, put hor-up
                            grid_buffer.get(start_i).set_preview(CHAR_HOR_UP_L);
                        }
                    }
                }
            }
            LineDirection::Horizontal => {
                let from = if from_col > to_col { to_col } else { from_col };
                let to = if from_col > to_col { from_col } else { to_col };
                for col in from..=to {
                    let i = from_row * cols + col;
                    grid_buffer.get(i).set_preview(CHAR_HOR_L);
                }

                if grid_buffer.get(start_i).read_content() == CHAR_VER_L {
                    let prev_i = (from_row - 1) * cols + from_col;
                    let next_i = (from_row + 1) * cols + from_col;
                    if grid_buffer.get(prev_i).read() == CHAR_SPACE {
                        if from_col < to_col {
                            // Draw right, put top-left corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_TL_L);
                        }
                        if from_col > to_col {
                            // Draw left, put top-right corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_TR_L);
                        }
                    } else if grid_buffer.get(next_i).read() == CHAR_SPACE {
                        if from_col < to_col {
                            // Draw right, put bottom-left corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_BL_L);
                        }
                        if from_col > to_col {
                            // Draw left, put bottom-right corner
                            grid_buffer.get(start_i).set_preview(CHAR_CORNER_BR_L);
                        }
                    } else {
                        if from_col < to_col {
                            // Draw right, put ver-right
                            grid_buffer.get(start_i).set_preview(CHAR_VER_RIGHT_L);
                        }
                        if from_col > to_col {
                            // Draw left, put ver-left
                            grid_buffer.get(start_i).set_preview(CHAR_VER_LEFT_L);
                        }
                    }
                }
            }
        }
    }

    fn commit(&mut self, grid_buffer: &mut crate::data::GridList) {
        grid_buffer.commit_all();
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
        false
    }
}

impl Shape for LineShape {}

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
