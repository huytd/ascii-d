use crate::{
    consts::{
        CHAR_CORNER_BL_L, CHAR_CORNER_BR_L, CHAR_CORNER_TL_L, CHAR_CORNER_TR_L, CHAR_HOR_DOWN_L,
        CHAR_HOR_L, CHAR_HOR_UP_L, CHAR_SPACE, CHAR_VER_L, CHAR_VER_LEFT_L, CHAR_VER_RIGHT_L,
    },
    data::GridCell,
    shapes::{
        line::{LineDirection, LineShape},
        ShapeList, ShapeRender,
    },
};

use super::ToolControl;

pub struct LineTool {}

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
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        let (cell_width, cell_height) = cell_size;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        shape_list.push(Box::new(LineShape::new(mouse_row, mouse_col)));
    }

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        if let Some(line) = shape_list.last_mut() {
            if let Some(mut line) = line.as_any_mut().downcast_mut::<LineShape>() {
                // TODO: Boundary check for row / col access
                let (cell_width, cell_height) = cell_size;
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
        event: &druid::MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
    }
}
