use druid::EventCtx;

use crate::{
    consts::CHAR_SPACE,
    data::{
        grid_list::GridList,
        history::{Version, HISTORY_MANAGER},
        shape_list::ShapeList,
    },
};

use super::{DrawingTools, ResizeOption, ToolControl, ToolsSize};

pub struct EraserTool {
    version: Version,
    last_cursor_position: Option<usize>,
    size: ToolsSize,
    _method: EraserMethod,
}

pub enum EraserAction {
    Clear,
    CheckEmpty,
}

pub enum EraserMethod {
    HistoryBased,
    //ShapeBased,
}

impl EraserTool {
    pub fn new() -> Self {
        Self {
            version: Version::new(),
            last_cursor_position: None,
            size: ToolsSize::Default,
            _method: EraserMethod::HistoryBased,
        }
    }

    pub fn increase_size(&mut self) {
        self.size = match self.size {
            ToolsSize::Default => ToolsSize::Small,
            ToolsSize::Small => ToolsSize::Medium,
            ToolsSize::Medium => ToolsSize::Large,
            ToolsSize::Large => ToolsSize::Large,
        };
    }

    pub fn decrease_size(&mut self) {
        self.size = match self.size {
            ToolsSize::Default => ToolsSize::Default,
            ToolsSize::Small => ToolsSize::Default,
            ToolsSize::Medium => ToolsSize::Small,
            ToolsSize::Large => ToolsSize::Medium,
        };
    }

    pub fn area_action(
        self: &mut Self,
        grid_list: &mut GridList,
        pos: usize,
        action: &EraserAction,
    ) -> bool {
        let (rows, cols) = grid_list.grid_size;
        let row = pos / cols;
        let col = pos % cols;

        let k = self.size as usize;

        for i in (row.saturating_sub(k))..=(row + k) {
            for j in (col.saturating_sub(k))..=(col + k) {
                if i >= rows || j >= cols {
                    continue;
                }

                let pos = i * cols + j;
                let from_content = grid_list.get(pos).read_content();

                match action {
                    EraserAction::Clear => {
                        grid_list.get(pos).clear();
                        self.version.push_without_overwrite(
                            pos,
                            from_content,
                            CHAR_SPACE,
                            DrawingTools::Eraser,
                        );
                    }
                    EraserAction::CheckEmpty => {
                        if !from_content.eq(&CHAR_SPACE) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    pub fn shape_action(
        self: &mut Self,
        grid_list: &mut GridList,
        pos: usize,
        action: &EraserAction,
    ) -> bool {
        let from_content = grid_list.get(pos).read_content();

        match action {
            EraserAction::Clear => unsafe {
                let history_index = HISTORY_MANAGER.get_history().iter().position(|version| {
                    version.get_edits().iter().any(|edit| {
                        edit.get_index() == pos && edit.get_tool() != DrawingTools::Select
                    })
                });

                if let Some(history_index) = history_index {
                    let version = HISTORY_MANAGER.get_history().get(history_index).unwrap();

                    let mut new_version = Version::new();
                    for edit in version.get_edits() {
                        grid_list.set(edit.get_index(), CHAR_SPACE);
                        new_version.push(
                            edit.get_index(),
                            edit.get_to(),
                            CHAR_SPACE,
                            DrawingTools::Eraser,
                        );
                    }

                    HISTORY_MANAGER.save_version(new_version);
                }
            },
            EraserAction::CheckEmpty => {
                if !from_content.eq(&CHAR_SPACE) {
                    return false;
                }
            }
        }

        true
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

        // if user is holding ctrl key --> erase shape
        if event.mods.ctrl() {
            if let Some(last_cursor_pos) = self.last_cursor_position {
                if i == last_cursor_pos
                    || self.shape_action(grid_list, i, &EraserAction::CheckEmpty)
                {
                    return;
                }
            }

            self.last_cursor_position = Some(i);
            self.shape_action(grid_list, i, &EraserAction::Clear);
            return;
        }

        // if user is not holding ctrl key --> erase area
        if let Some(last_cursor_pos) = self.last_cursor_position {
            if i == last_cursor_pos || self.area_action(grid_list, i, &EraserAction::CheckEmpty) {
                return;
            }
        }

        self.last_cursor_position = Some(i);
        self.area_action(grid_list, i, &EraserAction::Clear);
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

    fn resize(&mut self, option: ResizeOption) {
        match option {
            ResizeOption::Increase => self.increase_size(),
            ResizeOption::Decrease => self.decrease_size(),
        }
    }
}
