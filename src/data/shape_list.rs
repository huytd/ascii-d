use druid::{Point, Rect};

use crate::shapes::Shape;

use super::grid_list::GridList;

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

    pub fn find_shape_in_point(
        &mut self,
        point: Point,
        grid_list: &mut GridList,
    ) -> Option<&mut Box<dyn Shape>> {
        let (cell_width, cell_height) = grid_list.cell_size;
        let x = point.x / cell_width;
        let y = point.y / cell_height;
        let point = Point::new(y, x);

        for shape in self.data.iter_mut() {
            let (start_point, end_point) = shape.get_points();
            // TODO: Check if start_point and end_point is on a line, use different algorithm
            let shape_rect = Rect::from_points(start_point, end_point);
            if shape_rect.contains(point) {
                return Some(shape);
            }
        }

        None
    }

    pub fn find_shape_in_rect(
        &mut self,
        rect: Rect,
        grid_list: &mut GridList,
    ) -> Vec<&mut Box<dyn Shape>> {
        let mut result = vec![];
        let (cell_width, cell_height) = grid_list.cell_size;
        let start_row = rect.y0 / cell_height;
        let start_col = rect.x0 / cell_width;
        let end_row = rect.y1 / cell_height;
        let end_col = rect.x1 / cell_width;
        let selection_rect = Rect::new(start_row, start_col, end_row, end_col);

        for shape in self.data.iter_mut() {
            let (start_point, end_point) = shape.get_points();
            if selection_rect.contains(start_point) && selection_rect.contains(end_point) {
                result.push(shape);
            }
        }

        result
    }
}
