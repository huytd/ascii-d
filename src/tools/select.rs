use super::ToolControl;

pub struct SelectTool;

impl SelectTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToolControl for SelectTool {
    fn start(
        &mut self,
        _event: &druid::MouseEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        _grid_list: &mut crate::data::GridList,
    ) {
    }

    fn draw(
        &mut self,
        _event: &druid::MouseEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        _grid_list: &mut crate::data::GridList,
    ) {
    }

    fn input(
        &mut self,
        _event: &druid::KeyEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        _grid_list: &mut crate::data::GridList,
    ) {
    }

    fn end(
        &mut self,
        _event: &druid::MouseEvent,
        _shape_list: &mut crate::shapes::ShapeList,
        _grid_list: &mut crate::data::GridList,
    ) {
    }
}
