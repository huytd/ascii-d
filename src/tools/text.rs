use druid::KbKey;

use crate::{data::GridList, shapes::ShapeList};

use super::ToolControl;

pub struct TextTool {
    cursor_position: (usize, usize),
}

impl TextTool {
    pub fn new() -> Self {
        Self {
            cursor_position: (0, 0),
        }
    }
}

impl ToolControl for TextTool {
    fn start(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let (_, cols) = grid_list.grid_size;
        let row = (event.pos.y / cell_height) as usize;
        let col = (event.pos.x / cell_width) as usize;
        self.cursor_position = (row, col);
        let i = row * cols + col;
        grid_list.highlight(i);
    }

    fn draw(
        &mut self,
        _event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
    }

    fn end(
        &mut self,
        _event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
    }

    fn input(
        &mut self,
        event: &druid::KeyEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (_, cols) = grid_list.grid_size;
        let (row, col) = self.cursor_position;
        let i = row * cols + col;
        grid_list.highlight(i);

        match event.clone().key {
            KbKey::Character(c) => {
                let c = c.chars().next().unwrap();
                grid_list.get(i).set_content(c);
                self.cursor_position.1 += 1;
            }
            KbKey::Backspace => {
                grid_list.get(i).set_content(' ');
                self.cursor_position.1 -= 1;
            }
            KbKey::ArrowDown => {}
            _ => {}
        }

        // if let Some(text) = shape_list.data.last_mut() {
        //     if let Some(text) = text.as_any_mut().downcast_mut::<TextShape>() {
        //         match event.clone().key {
        //             KbKey::Character(c) => {
        //                 // text.push_char(c.chars().next().unwrap());
        //                 let c = c.chars().next().unwrap();
        //                 self.cursor_position.1 += 1;
        //             }
        //             KbKey::Backspace => {
        //                 text.pop_char();
        //                 self.cursor_position.1 -= 1;
        //             }
        //             KbKey::ArrowDown => {}
        //             _ => {}
        //         }

        //         let (_, cols) = grid_list.grid_size;
        //         let (row, col) = self.cursor_position;
        //         let i = row * cols + col;
        //         grid_list.highlight(i);
        //     }
        // }
    }
}
