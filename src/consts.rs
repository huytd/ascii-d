use druid::{Point, Selector};

pub const CHAR_HOR_L: char = '─';
pub const CHAR_VER_L: char = '│';
pub const CHAR_CORNER_TL_L: char = '┌';
pub const CHAR_CORNER_BL_L: char = '└';
pub const CHAR_CORNER_TR_L: char = '┐';
pub const CHAR_CORNER_BR_L: char = '┘';
pub const CHAR_HOR_UP_L: char = '┴';
pub const CHAR_HOR_DOWN_L: char = '┬';
pub const CHAR_VER_RIGHT_L: char = '├';
pub const CHAR_VER_LEFT_L: char = '┤';

pub const CHAR_ARROW_DOWN: char = '🭯';
pub const CHAR_ARROW_UP: char = '🭭';
pub const CHAR_ARROW_RIGHT: char = '►';
pub const CHAR_ARROW_LEFT: char = '◄';

pub fn is_arrowhead(c: char) -> bool {
    c.eq(&CHAR_ARROW_LEFT)
        || c.eq(&CHAR_ARROW_RIGHT)
        || c.eq(&CHAR_ARROW_UP)
        || c.eq(&CHAR_ARROW_DOWN)
}

pub const CHAR_SPACE: char = ' ';
pub const CHAR_NEWLINE: char = '\n';

pub const CANVAS_SIZE: f64 = 5000.0;

pub const BUTTON_HIGHLIGHT_COMMAND: Selector<String> = Selector::new("button-highlight");

pub const SELECTION_START_COMMAND: Selector<Point> = Selector::new("selection-start");
pub const SELECTION_END_COMMAND: Selector<Point> = Selector::new("selection-end");
pub const SELECTION_MOVE_COMMAND: Selector<Point> = Selector::new("selection-move");
