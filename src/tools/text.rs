use druid::{keyboard_types, KbKey};

use crate::{
    data::GridList,
    shapes::{text::TextShape, ShapeList, ShapeRender},
};

use super::ToolControl;

pub struct TextTool;

impl TextTool {
    pub fn new() -> Self {
        Self {}
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
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        shape_list.add_shape(Box::new(TextShape::new(mouse_row, mouse_col, "")));
    }

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
    }

    fn end(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
    }

    fn input(
        &mut self,
        event: &druid::KeyEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        if let Some(text) = shape_list.data.last_mut() {
            if let Some(mut text) = text.as_any_mut().downcast_mut::<TextShape>() {
                match event.clone().key {
                    KbKey::Character(c) => {
                        text.push_char(c.chars().next().unwrap());
                    }
                    KbKey::Backspace => {
                        text.pop_char();
                    }
                    KbKey::ArrowDown => {}
                    _ => {}
                }
            }
        }
    }
}
