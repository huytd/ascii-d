use std::any::Any;

use druid::{Point, Rect};

use crate::data::{grid_cell::GridCell, grid_list::GridList};

use self::rect::RectShape;

pub mod line;
pub mod rect;

pub trait ShapeRender {
    fn draw(&mut self, grid_buffer: &mut GridList);
    fn commit(&mut self, grid_buffer: &mut GridList);
    fn is_preview(&self) -> bool;
    fn is_manual_commit(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Shape: ShapeRender + std::fmt::Debug {}

pub struct ShapeList {
    pub data: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn draw(&mut self, grid_list: &mut GridList) {
        if let Some(shape) = self.data.last_mut() {
            if shape.is_preview() {
                shape.draw(grid_list);
            }
        }
    }

    pub fn commit(&mut self, grid_list: &mut GridList) {
        for shape in self.data.iter_mut() {
            if shape.is_preview() && !shape.is_manual_commit() {
                shape.commit(grid_list);
            }
        }
    }

    pub fn commit_all(&mut self, grid_list: &mut GridList) {
        for shape in self.data.iter_mut() {
            shape.commit(grid_list);
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.data.push(shape);
    }

    pub fn find_shape_in_rect(
        &mut self,
        rect: Rect,
        grid_list: &mut GridList,
    ) -> Vec<&mut RectShape> {
        let mut result = vec![];
        let (cell_width, cell_height) = grid_list.cell_size;
        let selection_start_row = rect.y0 / cell_height;
        let selection_start_col = rect.x0 / cell_width;
        let selection_end_row = rect.y1 / cell_height;
        let selection_end_col = rect.x1 / cell_width;
        let selection_rect = Rect::new(
            selection_start_row,
            selection_start_col,
            selection_end_row,
            selection_end_col,
        );

        // TODO: Make it work with LineShapes too

        for shape in self.data.iter_mut() {
            if let Some(shape) = shape.as_any_mut().downcast_mut::<RectShape>() {
                let start_point = Point::new(shape.start.0 as f64, shape.start.1 as f64);
                let end_point = Point::new(shape.end.0 as f64, shape.end.1 as f64);
                if selection_rect.contains(start_point) && selection_rect.contains(end_point) {
                    result.push(shape);
                }
            }
        }

        result
    }
}
