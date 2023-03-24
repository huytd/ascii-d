use druid::Point;

use crate::{
    consts::{
        CHAR_CORNER_BL_L, CHAR_CORNER_BR_L, CHAR_CORNER_TL_L, CHAR_CORNER_TR_L, CHAR_HOR_L,
        CHAR_VER_L,
    },
    data::grid_list::GridList,
};

use super::ShapeRender;

pub struct BlockShape {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub preview: bool,
    pub content: String,
}

impl BlockShape {
    pub fn new(row: usize, col: usize, content: String) -> Self {
        Self {
            start: (row, col),
            end: (row, col),
            preview: true,
            content: content,
        }
    }
}

impl_shape_for!(BlockShape);

impl ShapeRender for BlockShape {
    fn draw(&mut self, grid_buffer: &mut GridList) {
        let (row, col) = self.start;
        grid_buffer.discard_all();
        grid_buffer.put_preview_at(&self.content, row, col)
    }

    fn commit(&mut self, grid_buffer: &mut GridList) {
        grid_buffer.commit_all();
        self.preview = false;
    }

    fn is_preview(&self) -> bool {
        self.preview
    }

    fn is_manual_commit(&self) -> bool {
        false
    }
}
