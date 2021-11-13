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
        todo!()
    }

    fn end(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        todo!()
    }

    fn input(
        &mut self,
        event: &druid::KeyEvent,
        shape_list: &mut crate::shapes::ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        // Todo: Implement text input with the help of druid_shell::text
        // https://github.com/linebender/druid/blob/ac3e0a6601fabeefa56c1ca7f6e4f4096febf4b5/druid-shell/src/text.rs
    }
}
