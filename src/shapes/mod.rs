use std::any::Any;

use druid::Point;

use crate::data::grid_list::GridList;

pub mod line;
pub mod rect;

pub trait ShapeRender {
    fn draw(&mut self, grid_buffer: &mut GridList);
    fn commit(&mut self, grid_buffer: &mut GridList);
    fn is_preview(&self) -> bool;
    fn is_manual_commit(&self) -> bool;
}

pub trait Shape: ShapeRender {
    fn get_points(&self) -> (Point, Point);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
