use druid::MouseEvent;

use crate::tools::line::LineTool;

mod erase;
mod line;

pub trait ToolControl {
    fn start(&mut self, event: &MouseEvent, cell_size: (f64, f64), grid: (usize, usize));
    fn draw(
        &mut self,
        event: &MouseEvent,
        buffer: &mut Vec<char>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    );
    fn end(
        &mut self,
        event: &MouseEvent,
        buffer: &mut Vec<char>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    );
}

pub struct ToolManager {
    available_tools: Vec<Box<dyn ToolControl>>,
    current: usize,
}

impl ToolManager {
    pub fn new() -> Self {
        Self {
            available_tools: vec![Box::new(LineTool::new())],
            current: 0,
        }
    }

    pub fn set_tool(&mut self, index: usize) {
        self.current = index;
    }
}

impl ToolControl for ToolManager {
    fn start(&mut self, event: &MouseEvent, cell_size: (f64, f64), grid: (usize, usize)) {
        if let Some(tool) = self.available_tools.get_mut(self.current) {
            tool.start(event, cell_size, grid);
        }
    }

    fn draw(
        &mut self,
        event: &MouseEvent,
        buffer: &mut Vec<char>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        if let Some(tool) = self.available_tools.get_mut(self.current) {
            tool.draw(event, buffer, cell_size, grid);
        }
    }

    fn end(
        &mut self,
        event: &MouseEvent,
        buffer: &mut Vec<char>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        if let Some(tool) = self.available_tools.get_mut(self.current) {
            tool.draw(event, buffer, cell_size, grid);
        }
    }
}
