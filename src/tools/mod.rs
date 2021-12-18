use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use druid::{Data, KeyEvent, MouseEvent};

use crate::{
    data::GridList,
    shapes::ShapeList,
    tools::{line::LineTool, text::TextTool},
};

use self::{eraser::EraserTool, select::SelectTool};

pub mod eraser;
pub mod line;
pub mod select;
pub mod text;

#[derive(Clone, Copy, PartialEq, Data)]
pub enum DrawingTools {
    Select = 0,
    Line = 1,
    Text = 2,
    Eraser = 3,
}

impl Display for DrawingTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            DrawingTools::Select => "SELECT",
            DrawingTools::Line => "LINE",
            DrawingTools::Text => "TEXT",
            DrawingTools::Eraser => "ERASER",
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
    fn start(&mut self, event: &MouseEvent, shape_list: &mut ShapeList, grid_list: &mut GridList);
    fn draw(&mut self, event: &MouseEvent, shape_list: &mut ShapeList, grid_list: &mut GridList);
    fn input(&mut self, event: &KeyEvent, shape_list: &mut ShapeList, grid_list: &mut GridList);
    fn end(&mut self, event: &MouseEvent, shape_list: &mut ShapeList, grid_list: &mut GridList);
}

pub struct ToolManager {
    available_tools: Vec<Box<dyn ToolControl>>,
    current: DrawingTools,
}

impl ToolManager {
    pub fn new() -> Self {
        Self {
            available_tools: vec![
                Box::new(SelectTool::new()),
                Box::new(LineTool::new()),
                Box::new(TextTool::new()),
                Box::new(EraserTool::new()),
            ],
            current: DrawingTools::Select,
        }
    }

    pub fn set_tool(&mut self, tool: DrawingTools) {
        self.current = tool;
    }
}

impl ToolControl for ToolManager {
    fn start(&mut self, event: &MouseEvent, shape_list: &mut ShapeList, grid_list: &mut GridList) {
        self.available_tools[self.current].start(event, shape_list, grid_list);
    }

    fn draw(&mut self, event: &MouseEvent, shape_list: &mut ShapeList, grid_list: &mut GridList) {
        self.available_tools[self.current].draw(event, shape_list, grid_list);
    }

    fn end(&mut self, event: &MouseEvent, shape_list: &mut ShapeList, grid_list: &mut GridList) {
        self.available_tools[self.current].end(event, shape_list, grid_list);
    }

    fn input(&mut self, event: &KeyEvent, shape_list: &mut ShapeList, grid_list: &mut GridList) {
        self.available_tools[self.current].input(event, shape_list, grid_list);
    }
}
