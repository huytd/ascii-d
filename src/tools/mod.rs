use std::{
    cell,
    fmt::Display,
    ops::{Index, IndexMut},
};

use druid::{keyboard_types::KeyboardEvent, Event, KeyEvent, MouseEvent};

use crate::{
    shapes::ShapeList,
    tools::{line::LineTool, text::TextTool},
};

pub mod line;
pub mod text;

#[derive(Clone, Copy, PartialEq)]
pub enum DrawingTools {
    Line = 0,
    Text = 1,
}

impl Display for DrawingTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            DrawingTools::Line => "LINE",
            DrawingTools::Text => "TEXT",
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
    fn input(
        &mut self,
        event: &KeyEvent,
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
            available_tools: vec![Box::new(LineTool::new()), Box::new(TextTool::new())],
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

    fn input(
        &mut self,
        event: &KeyEvent,
        shape_list: &mut ShapeList,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        self.available_tools[self.current].input(event, shape_list, cell_size, grid);
    }
}
