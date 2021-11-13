use druid::{keyboard_types, KbKey};

use crate::shapes::text::TextShape;

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
        shape_list: &mut crate::shapes::ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        let (cell_width, cell_height) = cell_size;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        shape_list.push(Box::new(TextShape::new(mouse_row, mouse_col, "")));
    }

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
    }

    fn end(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
    }

    fn input(
        &mut self,
        event: &druid::KeyEvent,
        shape_list: &mut crate::shapes::ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        match event.clone().key {
            KbKey::Character(c) => {
                println!("PRESSED {}", c);
            }
            _ => {}
        }
    }
}
