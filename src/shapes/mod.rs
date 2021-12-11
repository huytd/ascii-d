use std::{any::Any, fmt::Debug, path::Iter, result::IterMut};

use crate::data::{GridCell, GridList};

pub mod line;
pub mod text;

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
}
