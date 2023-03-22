use druid::EventCtx;

use crate::{
    consts::CHAR_SPACE,
    data::{
        grid_list::GridList,
        history::{Version, HISTORY_MANAGER},
        shape_list::ShapeList,
    },
};

use super::ToolControl;

pub struct EraserTool {
    version: Version,
    last_cursor_position: Option<usize>,
}

impl EraserTool {
    pub fn new() -> Self {
        Self {
            version: Version::new(),
            last_cursor_position: None,
        }
    }
}

impl ToolControl for EraserTool {
    fn start(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
        self.last_cursor_position = None;
    }

    fn draw(
        &mut self,
        _ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let row = (event.pos.y / cell_height) as usize;
        let col = (event.pos.x / cell_width) as usize;
        let (_rows, cols) = grid_list.grid_size;
        let i = row * cols + col;
        if let Some(last_cursor_pos) = self.last_cursor_position {
            let from_content = grid_list.get(i).read_content();
            if i == last_cursor_pos || from_content.eq(&CHAR_SPACE) {
                return;
            }
        }
        self.last_cursor_position = Some(i);
        let cell = grid_list.get(i);
        self.version.push(i, cell.content, CHAR_SPACE);
        cell.clear();
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
        _ctx: &mut EventCtx,
        _event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
        unsafe {
            HISTORY_MANAGER.save_version(self.version.clone());
            self.version.clear();
        }
    }
}
