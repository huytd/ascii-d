use druid::{EventCtx, KbKey};

use crate::{data::grid_list::GridList, shapes::ShapeList};

use super::ToolControl;

pub struct TextTool {
    cursor_position: (usize, usize),
    last_edit_position: Option<(usize, usize)>,
}

impl TextTool {
    pub fn new() -> Self {
        Self {
            cursor_position: (0, 0),
            last_edit_position: None,
        }
    }
}

impl ToolControl for TextTool {
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
        let (_, cols) = grid_list.grid_size;

        match event.clone().key {
            KbKey::Character(c) => {
                if self.last_edit_position.is_none() {
                    self.last_edit_position = Some(self.cursor_position);
                }
                let c = c.chars().next().unwrap();
                let (row, col) = self.cursor_position;
                let i = row * cols + col;
                grid_list.get(i).set_content(c);
                self.cursor_position.1 += 1;
            }
            KbKey::Backspace => {
                self.cursor_position.1 -= 1;
                let (row, col) = self.cursor_position;
                let i = row * cols + col;
                grid_list.get(i).set_content(' ');
            }
            KbKey::ArrowDown => {
                self.cursor_position.0 += 1;
                self.last_edit_position = None;
            }
            KbKey::ArrowUp => {
                self.cursor_position.0 -= 1;
                self.last_edit_position = None;
            }
            KbKey::ArrowRight => {
                self.cursor_position.1 += 1;
                self.last_edit_position = None;
            }
            KbKey::ArrowLeft => {
                self.cursor_position.1 -= 1;
                self.last_edit_position = None;
            }
            KbKey::Enter => {
                if let Some(pos) = self.last_edit_position {
                    self.cursor_position = (pos.0 + 1, pos.1);
                    self.last_edit_position = Some(self.cursor_position);
                }
            }
            _ => {}
        }

        let (row, col) = self.cursor_position;
        let i = row * cols + col;
        grid_list.highlight(i);
    }
}
