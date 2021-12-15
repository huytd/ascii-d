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
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut crate::data::GridList,
    ) {
    }

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut crate::data::GridList,
    ) {
    }

    fn input(
        &mut self,
        event: &druid::KeyEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut crate::data::GridList,
    ) {
    }

    fn end(
        &mut self,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut crate::data::GridList,
    ) {
    }
}
