use druid::EventCtx;

use crate::data::grid_list::GridList;

use super::ToolControl;

pub struct EraserTool;

impl EraserTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToolControl for EraserTool {
    fn start(
        &mut self,
        ctx: &mut EventCtx,
        _event: &druid::MouseEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        _grid_list: &mut GridList,
    ) {
    }

    fn draw(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let row = (event.pos.y / cell_height) as usize;
        let col = (event.pos.x / cell_width) as usize;
        let (_rows, cols) = grid_list.grid_size;
        let i = row * cols + col;
        grid_list.get(i).clear();
    }

    fn input(
        &mut self,
        ctx: &mut EventCtx,
        _event: &druid::KeyEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        _grid_list: &mut GridList,
    ) {
    }

    fn end(
        &mut self,
        ctx: &mut EventCtx,
        _event: &druid::MouseEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        _grid_list: &mut GridList,
    ) {
    }
}
