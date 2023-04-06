use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use druid::{Data, EventCtx, KeyEvent, MouseEvent};

use crate::{
    data::{grid_list::GridList, shape_list::ShapeList},
    tools::{line::LineTool, text::TextTool},
};

use self::{eraser::EraserTool, rect::RectTool, select::SelectTool};

pub mod eraser;
pub mod line;
pub mod rect;
pub mod select;
pub mod text;

#[derive(Clone, Copy, PartialEq, Data, Debug)]
pub enum DrawingTools {
    Select = 0,
    Line = 1,
    Text = 2,
    Eraser = 3,
    Rect = 4,
}

#[derive(Clone, Copy, PartialEq, Data, Debug)]
pub enum ToolsSize {
    Default = 0,
    Small = 1 << 0,
    Medium = 1 << 1,
    Large = 1 << 2,
}

pub enum ResizeOption {
    Increase,
    Decrease,
}

impl Display for DrawingTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            DrawingTools::Select => "SELECT",
            DrawingTools::Line => "LINE",
            DrawingTools::Text => "TEXT",
            DrawingTools::Eraser => "ERASER",
            DrawingTools::Rect => "RECTANGLE",
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
    fn draw(
        &mut self,
        ctx: &mut EventCtx,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    );
    fn end(
        &mut self,
        ctx: &mut EventCtx,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    );
    fn input(
        &mut self,
        ctx: &mut EventCtx,
        event: &KeyEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    );
    fn start(
        &mut self,
        ctx: &mut EventCtx,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    );

    fn resize(&mut self, option: ResizeOption);
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
                Box::new(RectTool::new()),
            ],
            current: DrawingTools::Select,
        }
    }

    pub fn set_tool(&mut self, tool: DrawingTools) {
        self.current = tool;
    }
}

impl ToolControl for ToolManager {
    fn start(
        &mut self,
        ctx: &mut EventCtx,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        self.available_tools[self.current].start(ctx, event, shape_list, grid_list);
    }

    fn draw(
        &mut self,
        ctx: &mut EventCtx,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        self.available_tools[self.current].draw(ctx, event, shape_list, grid_list);
    }

    fn input(
        &mut self,
        ctx: &mut EventCtx,
        event: &KeyEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        self.available_tools[self.current].input(ctx, event, shape_list, grid_list);
    }

    fn end(
        &mut self,
        ctx: &mut EventCtx,
        event: &MouseEvent,
        shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        self.available_tools[self.current].end(ctx, event, shape_list, grid_list);
    }

    fn resize(&mut self, option: ResizeOption) {
        self.available_tools[self.current].resize(option);
    }
}
