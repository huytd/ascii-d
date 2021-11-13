use std::{any::Any, fmt::Debug};

use crate::data::GridCell;

pub mod line;
pub mod text;

pub trait ShapeRender {
    fn draw(
        &mut self,
        grid_buffer: &mut Vec<GridCell>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    );
    fn commit(&mut self, grid_buffer: &mut Vec<GridCell>);
    fn is_preview(&self) -> bool;
    fn is_manual_commit(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Shape: ShapeRender + std::fmt::Debug {}

pub type ShapeList = Vec<Box<dyn Shape>>;
