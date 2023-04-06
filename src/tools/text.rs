use druid::{EventCtx, KbKey};

use crate::data::{
    grid_list::GridList,
    history::{Version, HISTORY_MANAGER},
    shape_list::ShapeList,
};

use super::{ResizeOption, ToolControl};

pub struct TextTool {
    cursor_position: (usize, usize),
    last_edit_position: Option<(usize, usize)>,
    version: Version,
}

impl TextTool {
    pub fn new() -> Self {
        Self {
            cursor_position: (0, 0),
            last_edit_position: None,
            version: Version::new(),
        }
    }

    fn cursor_step_forward(&mut self, rows: usize, cols: usize) {
        match self.cursor_position {
            (r, c) if r >= rows - 1 && c >= cols - 1 => {}
            (_, c) if c >= cols - 1 => self.cursor_position = (self.cursor_position.0 + 1, 0),
            _ => self.cursor_position.1 += 1,
        }
    }

    fn cursor_step_backward(&mut self, cols: usize) {
        match self.cursor_position {
            (0, 0) => {}
            (_, 0) => self.cursor_position = (self.cursor_position.0 - 1, cols - 1),
            _ => self.cursor_position.1 -= 1,
        }
    }
}

impl ToolControl for TextTool {
    fn draw(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &druid::MouseEvent,
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
    }

    fn input(
        &mut self,
        _ctx: &mut EventCtx,
        event: &druid::KeyEvent,
        _shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (rows, cols) = grid_list.grid_size;

        match event.clone().key {
            KbKey::Character(c) => {
                if self.last_edit_position.is_none() {
                    self.last_edit_position = Some(self.cursor_position);
                }
                let c = c.chars().next().unwrap();
                let (row, col) = self.cursor_position;
                let i = row * cols + col;
                let cell = grid_list.get(i);
                let from_content = cell.content;
                let to_content = c;
                cell.set_content(c);
                self.cursor_step_forward(rows, cols);
                self.version.push(i, from_content, to_content);
            }
            KbKey::Backspace => {
                match self.cursor_position {
                    (0, 0) => {}
                    (_, 0) => {
                        self.cursor_position.0 -= 1;
                        self.cursor_position.1 = cols - 1;
                    }
                    _ => self.cursor_position.1 -= 1,
                }
                let (row, col) = self.cursor_position;
                let i = row * cols + col;
                let cell = grid_list.get(i);
                let from_content = cell.content;
                let to_content = ' ';
                cell.set_content(' ');
                self.version.push(i, from_content, to_content);
            }
            KbKey::ArrowDown => {
                if self.cursor_position.0 < rows - 1 {
                    self.cursor_position.0 += 1;
                    self.last_edit_position = None;
                }
            }
            KbKey::ArrowUp => {
                if self.cursor_position.0 > 0 {
                    self.cursor_position.0 -= 1;
                    self.last_edit_position = None;
                }
            }
            KbKey::ArrowRight => {
                self.cursor_step_forward(rows, cols);
                self.last_edit_position = None;
            }
            KbKey::ArrowLeft => {
                self.cursor_step_backward(cols);
                self.last_edit_position = None;
            }
            KbKey::Enter => {
                if let Some(pos) = self.last_edit_position {
                    if pos.0 < rows - 1 {
                        self.cursor_position = (pos.0 + 1, pos.1);
                        self.last_edit_position = Some(self.cursor_position);
                    }
                }
            }
            _ => {}
        }

        let (row, col) = self.cursor_position;
        let i = row * cols + col;
        grid_list.highlight(i);

        unsafe {
            HISTORY_MANAGER.save_version(self.version.clone());
            self.version = Version::new();
        }
    }

    fn start(
        &mut self,
        _ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let (_, cols) = grid_list.grid_size;
        let row = (event.pos.y / cell_height) as usize;
        let col = (event.pos.x / cell_width) as usize;
        self.cursor_position = (row, col);
        self.last_edit_position = Some(self.cursor_position);
        let i = row * cols + col;
        grid_list.highlight(i);
    }

    fn resize(&mut self, option: ResizeOption) {}
}
