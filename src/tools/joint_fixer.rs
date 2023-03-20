use druid::EventCtx;

use crate::{
    consts::{
        CHAR_CORNER_BL_L, CHAR_CORNER_BR_L, CHAR_CORNER_TL_L, CHAR_CORNER_TR_L, CHAR_CROSS,
        CHAR_HOR_DOWN_L, CHAR_HOR_L, CHAR_HOR_UP_L, CHAR_SPACE, CHAR_VER_L, CHAR_VER_LEFT_L,
        CHAR_VER_RIGHT_L,
    },
    data::{
        grid_list::GridList,
        history::{Version, HISTORY_MANAGER},
        shape_list::ShapeList,
    },
};

use super::ToolControl;

pub struct JointFixerTool {
    version: Version,
}

macro_rules! idx_to_opt_char {
    ($var:ident,$input:ident,$g:ident) => {
        let $var = if let Some(i) = $input {
            match $g.get(i).read_content() {
                CHAR_SPACE => None,
                c => Some(c),
            }
        } else {
            None
        };
    };
}

macro_rules! UP_CHARS {
    () => {
        CHAR_VER_L
            | CHAR_CORNER_TR_L
            | CHAR_CORNER_TL_L
            | CHAR_HOR_DOWN_L
            | CHAR_VER_RIGHT_L
            | CHAR_VER_LEFT_L
            | CHAR_CROSS
    };
}

macro_rules! DOWN_CHARS {
    () => {
        CHAR_VER_L
            | CHAR_CORNER_BL_L
            | CHAR_CORNER_BR_L
            | CHAR_HOR_UP_L
            | CHAR_VER_RIGHT_L
            | CHAR_VER_LEFT_L
            | CHAR_CROSS
    };
}

macro_rules! LEFT_CHARS {
    () => {
        CHAR_HOR_L
            | CHAR_CORNER_TL_L
            | CHAR_CORNER_BL_L
            | CHAR_HOR_UP_L
            | CHAR_HOR_DOWN_L
            | CHAR_VER_RIGHT_L
            | CHAR_CROSS
    };
}

macro_rules! RIGHT_CHARS {
    () => {
        CHAR_HOR_L
            | CHAR_CORNER_TR_L
            | CHAR_CORNER_BR_L
            | CHAR_HOR_UP_L
            | CHAR_HOR_DOWN_L
            | CHAR_VER_LEFT_L
            | CHAR_CROSS
    };
}

impl JointFixerTool {
    pub fn new() -> Self {
        Self {
            version: Version::new(),
        }
    }

    fn calculate_content(
        &self,
        grid_list: &mut GridList,
        u: Option<usize>,
        d: Option<usize>,
        l: Option<usize>,
        r: Option<usize>,
    ) -> Option<char> {
        idx_to_opt_char!(uc, u, grid_list);
        idx_to_opt_char!(dc, d, grid_list);
        idx_to_opt_char!(lc, l, grid_list);
        idx_to_opt_char!(rc, r, grid_list);

        match (uc, dc, lc, rc) {
            (None, None, None, None) => None,
            // Vertical
            // (Some(UP_CHARS!()), None, None, None) => Some(CHAR_VER_L),
            // (None, Some(DOWN_CHARS!()), None, None) => Some(CHAR_VER_L),
            (Some(UP_CHARS!()), Some(DOWN_CHARS!()), l, r)
                if !matches!((l, r), (Some(LEFT_CHARS!()), _) | (_, Some(RIGHT_CHARS!()))) =>
            {
                Some(CHAR_VER_L)
            }
            // Horizontal
            // (None, None, Some(LEFT_CHARS!()), None) => Some(CHAR_HOR_L),
            // (None, None, None, Some(RIGHT_CHARS!())) => Some(CHAR_HOR_L),
            (u, d, Some(LEFT_CHARS!()), Some(RIGHT_CHARS!()))
                if !matches!((u, d), (Some(UP_CHARS!()), _) | (_, Some(DOWN_CHARS!()))) =>
            {
                Some(CHAR_HOR_L)
            }
            // Two joint
            (Some(UP_CHARS!()), d, Some(LEFT_CHARS!()), r)
                if !matches!((d, r), (Some(DOWN_CHARS!()), _) | (_, Some(RIGHT_CHARS!()))) =>
            {
                Some(CHAR_CORNER_BR_L)
            }
            (Some(UP_CHARS!()), d, l, Some(RIGHT_CHARS!()))
                if !matches!((d, l), (Some(DOWN_CHARS!()), _) | (_, Some(LEFT_CHARS!()))) =>
            {
                Some(CHAR_CORNER_BL_L)
            }
            (u, Some(DOWN_CHARS!()), Some(LEFT_CHARS!()), r)
                if !matches!((u, r), (Some(UP_CHARS!()), _) | (_, Some(RIGHT_CHARS!()))) =>
            {
                Some(CHAR_CORNER_TR_L)
            }
            (u, Some(DOWN_CHARS!()), l, Some(RIGHT_CHARS!()))
                if !matches!((u, l), (Some(UP_CHARS!()), _) | (_, Some(LEFT_CHARS!()))) =>
            {
                Some(CHAR_CORNER_TL_L)
            }
            // Three joint
            (Some(UP_CHARS!()), Some(DOWN_CHARS!()), Some(LEFT_CHARS!()), r)
                if !matches!(r, Some(RIGHT_CHARS!())) =>
            {
                Some(CHAR_VER_LEFT_L)
            }
            (Some(UP_CHARS!()), Some(DOWN_CHARS!()), l, Some(RIGHT_CHARS!()))
                if !matches!(l, Some(LEFT_CHARS!())) =>
            {
                Some(CHAR_VER_RIGHT_L)
            }
            (Some(UP_CHARS!()), d, Some(LEFT_CHARS!()), Some(RIGHT_CHARS!()))
                if !matches!(d, Some(DOWN_CHARS!())) =>
            {
                Some(CHAR_HOR_UP_L)
            }
            (u, Some(DOWN_CHARS!()), Some(LEFT_CHARS!()), Some(RIGHT_CHARS!()))
                if !matches!(u, Some(UP_CHARS!())) =>
            {
                Some(CHAR_HOR_DOWN_L)
            }
            // Four joint
            (Some(UP_CHARS!()), Some(DOWN_CHARS!()), Some(LEFT_CHARS!()), Some(RIGHT_CHARS!())) => {
                Some(CHAR_CROSS)
            }
            _ => None,
        }
    }
}

impl ToolControl for JointFixerTool {
    fn start(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
    }

    fn draw(
        &mut self,
        _ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let row = (event.pos.y / cell_height) as usize;
        let col = (event.pos.x / cell_width) as usize;
        let (rows, cols) = grid_list.grid_size;
        let i = row * cols + col;

        let up_i = match row {
            0 => None,
            _ => Some((row - 1) * cols + col),
        };

        let down_i = match row {
            r if r >= rows - 1 => None,
            _ => Some((row + 1) * cols + col),
        };

        let left_i = match col {
            0 => None,
            _ => Some(i - 1),
        };

        let right_i = match col {
            c if c >= cols - 1 => None,
            _ => Some(i + 1),
        };

        if let Some(content) = self.calculate_content(grid_list, up_i, down_i, left_i, right_i) {
            let cell = grid_list.get(i);
            let from_content = cell.read_content();
            self.version.push(i, from_content, content);
            cell.set_content(content);
        }
    }

    fn end(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &druid::MouseEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
        unsafe {
            HISTORY_MANAGER.save_version(self.version.clone());
        }
    }

    fn input(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &druid::KeyEvent,
        _shape_list: &mut ShapeList,
        _grid_list: &mut GridList,
    ) {
    }
}
