use crate::{
    consts::{
        CHAR_CORNER_BL_L, CHAR_CORNER_BR_L, CHAR_CORNER_TL_L, CHAR_CORNER_TR_L, CHAR_HOR_DOWN_L,
        CHAR_HOR_L, CHAR_HOR_UP_L, CHAR_SPACE, CHAR_VER_L, CHAR_VER_LEFT_L, CHAR_VER_RIGHT_L,
    },
    data::GridCell,
};

use super::ToolControl;

pub struct LineTool {
    from: Option<(usize, usize)>,
}

impl LineTool {
    pub fn new() -> Self {
        Self { from: None }
    }
}

impl ToolControl for LineTool {
    fn start(&mut self, event: &druid::MouseEvent, cell_size: (f64, f64), grid: (usize, usize)) {
        let (cell_width, cell_height) = cell_size;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        self.from = Some((mouse_row, mouse_col));
    }

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        buffer: &mut Vec<GridCell>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        // TODO: Boundary check for row / col access
        let (cell_width, cell_height) = cell_size;
        let (rows, cols) = grid;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        if let Some((from_row, from_col)) = self.from {
            let d_row = (mouse_row as isize - from_row as isize).abs();
            let d_col = (mouse_col as isize - from_col as isize).abs();

            for cell in buffer.iter_mut() {
                if cell.preview.is_some() {
                    cell.discard();
                }
            }

            if d_row > d_col {
                // Draw vertical line
                let from = if from_row > mouse_row {
                    mouse_row
                } else {
                    from_row
                };
                let to = if from_row > mouse_row {
                    from_row
                } else {
                    mouse_row
                };
                for row in from..=to {
                    let i = row * cols + from_col;
                    buffer[i].set_preview(CHAR_VER_L);
                }
            } else {
                // Draw horizontal line
                let from = if from_col > mouse_col {
                    mouse_col
                } else {
                    from_col
                };
                let to = if from_col > mouse_col {
                    from_col
                } else {
                    mouse_col
                };
                for col in from..=to {
                    let i = from_row * cols + col;
                    buffer[i].set_preview(CHAR_HOR_L);
                }
            }

            // Fix the corner
            let start_i = from_row * cols + from_col;
            if buffer[start_i].read_content() == CHAR_HOR_L {
                let prev_i = from_row * cols + (from_col - 1);
                let next_i = from_row * cols + (from_col + 1);
                if buffer[prev_i].read() == CHAR_SPACE {
                    if from_row < mouse_row {
                        // Draw down, put top-left corner
                        buffer[start_i].set_preview(CHAR_CORNER_TL_L);
                    }
                    if from_row > mouse_row {
                        // Draw up, put bottom-left corner
                        buffer[start_i].set_preview(CHAR_CORNER_BL_L);
                    }
                } else if buffer[next_i].read() == CHAR_SPACE {
                    if from_row < mouse_row {
                        // Draw down, put top-right corner
                        buffer[start_i].set_preview(CHAR_CORNER_TR_L);
                    }
                    if from_row > mouse_row {
                        // Draw up, put bottom-right corner
                        buffer[start_i].set_preview(CHAR_CORNER_BR_L);
                    }
                } else {
                    if from_row < mouse_row {
                        // Draw down, put hor-down
                        buffer[start_i].set_preview(CHAR_HOR_DOWN_L);
                    }
                    if from_row > mouse_row {
                        // Draw up, put hor-up
                        buffer[start_i].set_preview(CHAR_HOR_UP_L);
                    }
                }
            }
            if buffer[start_i].read_content() == CHAR_VER_L {
                let prev_i = (from_row - 1) * cols + from_col;
                let next_i = (from_row + 1) * cols + from_col;
                if buffer[prev_i].read() == CHAR_SPACE {
                    if from_col < mouse_col {
                        // Draw right, put top-left corner
                        buffer[start_i].set_preview(CHAR_CORNER_TL_L);
                    }
                    if from_col > mouse_col {
                        // Draw left, put top-right corner
                        buffer[start_i].set_preview(CHAR_CORNER_TR_L);
                    }
                } else if buffer[next_i].read() == CHAR_SPACE {
                    if from_col < mouse_col {
                        // Draw right, put bottom-left corner
                        buffer[start_i].set_preview(CHAR_CORNER_BL_L);
                    }
                    if from_col > mouse_col {
                        // Draw left, put bottom-right corner
                        buffer[start_i].set_preview(CHAR_CORNER_BR_L);
                    }
                } else {
                    if from_col < mouse_col {
                        // Draw right, put ver-right
                        buffer[start_i].set_preview(CHAR_VER_RIGHT_L);
                    }
                    if from_col > mouse_col {
                        // Draw left, put ver-left
                        buffer[start_i].set_preview(CHAR_VER_LEFT_L);
                    }
                }
            }
        }
    }

    fn end(
        &mut self,
        event: &druid::MouseEvent,
        buffer: &mut Vec<GridCell>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        self.from = None;
        for cell in buffer {
            if cell.preview.is_some() {
                cell.commit();
            }
        }
    }
}
