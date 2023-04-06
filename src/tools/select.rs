use druid::EventCtx;
use std::ops::Sub;

use crate::shapes::block::BlockShape;
use crate::{
    consts::{SELECTION_END_COMMAND, SELECTION_MOVE_COMMAND, SELECTION_START_COMMAND},
    data::{grid_list::GridList, shape_list::ShapeList},
};

use super::{ResizeOption, ToolControl};

pub struct SelectTool {
    is_selecting: bool,
    offset_row: usize,
    offset_col: usize,
}

impl SelectTool {
    pub fn new() -> Self {
        Self {
            is_selecting: false,
            offset_row: 0,
            offset_col: 0,
        }
    }
}

impl ToolControl for SelectTool {
    fn start(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let (_, cols) = grid_list.grid_size;
        let row = (event.pos.y / cell_height) as usize;
        let col = (event.pos.x / cell_width) as usize;
        let i = row * cols + col;
        if !grid_list.get(i).highlighted {
            ctx.submit_command(SELECTION_START_COMMAND.with(event.pos));
            self.is_selecting = true;
        } else {
            if let Some(((sel_row, sel_col), _)) = grid_list.current_selection {
                // Calculate the offset between current mouse pos and selection rect
                self.offset_row = (row as isize - sel_row as isize).abs() as usize;
                self.offset_col = (col as isize - sel_col as isize).abs() as usize;
                // Create new block shape here
                let block_content = grid_list.get_highlighted_content();
                grid_list.erase_highlighted();
                grid_list.clear_all_highlight();
                shape_list.add_shape(Box::new(BlockShape::new(row, col, block_content)));
            }
        }
    }

    fn draw(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        if self.is_selecting {
            ctx.submit_command(SELECTION_MOVE_COMMAND.with(event.pos));
        } else {
            if let Some(block) = shape_list.data.last_mut() {
                if let Some(mut block) = block.as_any_mut().downcast_mut::<BlockShape>() {
                    let (cell_width, cell_height) = grid_list.cell_size;
                    let mouse_row = (event.pos.y / cell_height) as usize;
                    let mouse_col = (event.pos.x / cell_width) as usize;
                    let shape_row = mouse_row.saturating_sub(self.offset_row);
                    let shape_col = mouse_col.saturating_sub(self.offset_col);
                    block.start = (shape_row, shape_col);
                }
            }
        }
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
        self.is_selecting = false;
    }

    fn resize(&mut self, option: ResizeOption) {}
}
