use druid::EventCtx;

use crate::{
    consts::{SELECTION_END_COMMAND, SELECTION_MOVE_COMMAND, SELECTION_START_COMMAND},
    data::{grid_list::GridList, shape_list::ShapeList},
};

use super::ToolControl;

pub struct SelectTool;

impl SelectTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToolControl for SelectTool {
    fn start(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
        ctx.submit_command(SELECTION_START_COMMAND.with(event.pos));
    }

    fn draw(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
        ctx.submit_command(SELECTION_MOVE_COMMAND.with(event.pos));
    }

    fn input(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &druid::KeyEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
    }

    fn end(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
        ctx.submit_command(SELECTION_END_COMMAND.with(event.pos));
    }
}
