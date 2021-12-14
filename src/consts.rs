use druid::Selector;

use crate::tools::DrawingTools;

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

pub const CHAR_SPACE: char = ' ';
pub const CHAR_NEWLINE: char = '\n';

pub const CANVAS_SIZE: f64 = 5000.0;

pub const BUTTON_HIGHLIGHT_COMMAND: Selector<String> = Selector::new("button-highlight");
