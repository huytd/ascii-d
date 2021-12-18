use crate::{
    data::GridList,
    shapes::{
        line::{LineDirection, LineShape},
        ShapeList,
    },
};

use super::ToolControl;

pub struct LineTool;

impl LineTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToolControl for LineTool {
    fn start(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        shape_list.add_shape(Box::new(LineShape::new(mouse_row, mouse_col)));
    }

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        if let Some(line) = shape_list.data.last_mut() {
            if let Some(mut line) = line.as_any_mut().downcast_mut::<LineShape>() {
                // TODO: Boundary check for row / col access
                let (cell_width, cell_height) = grid_list.cell_size;
                let mouse_row = (event.pos.y / cell_height) as usize;
                let mouse_col = (event.pos.x / cell_width) as usize;
                let (from_row, from_col) = line.start;
                let d_row = (mouse_row as isize - from_row as isize).abs();
                let d_col = (mouse_col as isize - from_col as isize).abs();

                if d_row > d_col {
                    // Draw vertical line
                    line.end = (mouse_row, from_col);
                    line.direction = LineDirection::Vertical;
                } else {
                    // Draw horizontal line
                    line.end = (from_row, mouse_col);
                    line.direction = LineDirection::Horizontal;
                }
            }
        }
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
        _event: &druid::KeyEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
    }
}
