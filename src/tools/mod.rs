use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use druid::MouseEvent;

use crate::{shapes::ShapeList, tools::line::LineTool};

mod line;

#[derive(Clone, Copy, PartialEq)]
pub enum DrawingTools {
    Line = 0,
}

impl Display for DrawingTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            DrawingTools::Line => "LINE",
        };
        write!(f, "{}", op)
    }
}

impl<T> Index<DrawingTools> for Vec<T> {
    type Output = T;

    fn index(&self, index: DrawingTools) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<DrawingTools> for Vec<T> {
    fn index_mut(&mut self, index: DrawingTools) -> &mut T {
        &mut self[index as usize]
    }
}

pub trait ToolControl {
    fn start(
        &mut self,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    );
    fn draw(
        &mut self,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    );
    fn end(
        &mut self,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    );
}

pub struct ToolManager {
    available_tools: Vec<Box<dyn ToolControl>>,
    current: DrawingTools,
}

impl ToolManager {
    pub fn new() -> Self {
        Self {
            available_tools: vec![Box::new(LineTool::new())],
            current: DrawingTools::Line,
        }
    }

    pub fn set_tool(&mut self, tool: DrawingTools) {
        self.current = tool;
    }

    pub fn get_active_tool(&self) -> DrawingTools {
        return self.current;
    }
}

impl ToolControl for ToolManager {
    fn start(
        &mut self,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        self.available_tools[self.current].start(event, shape_list, cell_size, grid);
    }

    fn draw(
        &mut self,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        self.available_tools[self.current].draw(event, shape_list, cell_size, grid);
    }

    fn end(
        &mut self,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        self.available_tools[self.current].end(event, shape_list, cell_size, grid);
    }
}
