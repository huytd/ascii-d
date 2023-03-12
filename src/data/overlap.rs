use crate::consts::{
    CHAR_CROSS, CHAR_HOR_DOWN_L, CHAR_HOR_L, CHAR_HOR_UP_L, CHAR_VER_L, CHAR_VER_LEFT_L,
    CHAR_VER_RIGHT_L,
};
use crate::shapes::line::LineDirection;

fn process_hor_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    // TODO: Handle the transform to corner case
    if incoming == CHAR_VER_L {
        return if let Some(start_direction) = start_direction {
            match start_direction {
                LineDirection::UpToDown => CHAR_HOR_DOWN_L,
                LineDirection::DownToUp => CHAR_HOR_UP_L,
                _ => unreachable!(),
            }
        } else {
            CHAR_CROSS
        };
    }
    incoming
}

fn process_ver_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    // TODO: Handle the transform to corner case
    if incoming == CHAR_HOR_L {
        return if let Some(start_direction) = start_direction {
            match start_direction {
                LineDirection::RightToLeft => CHAR_VER_RIGHT_L,
                LineDirection::LeftToRight => CHAR_VER_LEFT_L,
                _ => unreachable!(),
            }
        } else {
            CHAR_CROSS
        };
    }
    incoming
}

fn process_corner_tl_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_corner_bl_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_corner_tr_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_corner_br_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_hor_up_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_hor_down_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_ver_right_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_ver_left_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_arrow_down(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_arrow_up(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_arrow_right(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_arrow_left(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

fn process_cross(start_direction: Option<LineDirection>, incoming: char) -> char {
    if let Some(start_direction) = start_direction {
        // Is start of a line
    } else {
        // Is a point during the line
    }
    incoming
}

pub fn calculate_cell_content(
    start_direction: Option<LineDirection>,
    current: char,
    incoming: char,
) -> char {
    // Is overlapping character or not?
    if !current.is_whitespace() {
        return match current {
            CHAR_HOR_L => process_hor_l(start_direction, incoming),
            CHAR_VER_L => process_ver_l(start_direction, incoming),
            CHAR_CORNER_TL_L => process_corner_tl_l(start_direction, incoming),
            CHAR_CORNER_BL_L => process_corner_bl_l(start_direction, incoming),
            CHAR_CORNER_TR_L => process_corner_tr_l(start_direction, incoming),
            CHAR_CORNER_BR_L => process_corner_br_l(start_direction, incoming),
            CHAR_HOR_UP_L => process_hor_up_l(start_direction, incoming),
            CHAR_HOR_DOWN_L => process_hor_down_l(start_direction, incoming),
            CHAR_VER_RIGHT_L => process_ver_right_l(start_direction, incoming),
            CHAR_VER_LEFT_L => process_ver_left_l(start_direction, incoming),
            CHAR_ARROW_DOWN => process_arrow_down(start_direction, incoming),
            CHAR_ARROW_UP => process_arrow_up(start_direction, incoming),
            CHAR_ARROW_RIGHT => process_arrow_right(start_direction, incoming),
            CHAR_ARROW_LEFT => process_arrow_left(start_direction, incoming),
            CHAR_CROSS => process_cross(start_direction, incoming),
            _ => incoming,
        };
    }
    incoming
}
