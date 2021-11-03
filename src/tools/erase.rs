use crate::data::GridCell;

use super::ToolControl;

pub struct EraseTool;

impl EraseTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToolControl for EraseTool {
    fn start(&mut self, event: &druid::MouseEvent, cell_size: (f64, f64), grid: (usize, usize)) {}

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        buffer: &mut Vec<GridCell>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        let (cell_width, cell_height) = cell_size;
        let (rows, cols) = grid;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        let i = mouse_row * cols + mouse_col;
        buffer[i].clear();
    }

    fn end(
        &mut self,
        event: &druid::MouseEvent,
        buffer: &mut Vec<GridCell>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
    }
}
