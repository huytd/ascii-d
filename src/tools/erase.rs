use super::ToolControl;

pub struct EraseTool;

impl ToolControl for EraseTool {
    fn start(&mut self, event: &druid::MouseEvent, cell_size: (f64, f64), grid: (usize, usize)) {
        todo!()
    }

    fn draw( &mut self, event: &druid::MouseEvent, buffer: &mut Vec<char>, cell_size: (f64, f64), grid: (usize, usize),) {
        todo!()
    }

    fn end(&mut self, event: &druid::MouseEvent, buffer: &mut Vec<char>, cell_size: (f64, f64), grid: (usize, usize)) {
        todo!()
    }
}