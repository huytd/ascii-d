use crate::consts::{
    CHAR_ARROW_DOWN, CHAR_ARROW_LEFT, CHAR_ARROW_RIGHT, CHAR_ARROW_UP, CHAR_CORNER_BL_L,
    CHAR_CORNER_BR_L, CHAR_CORNER_TL_L, CHAR_CORNER_TR_L, CHAR_CROSS, CHAR_HOR_DOWN_L, CHAR_HOR_L,
    CHAR_HOR_UP_L, CHAR_NEWLINE, CHAR_SPACE, CHAR_VER_L, CHAR_VER_LEFT_L, CHAR_VER_RIGHT_L,
};
use crate::shapes::line::LineDirection;

fn process_hor_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    // TODO: Handle the transform to corner case
    match incoming {
        CHAR_VER_L => {
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
        CHAR_CORNER_BL_L | CHAR_CORNER_BR_L => CHAR_HOR_UP_L,
        CHAR_CORNER_TL_L | CHAR_CORNER_TR_L => CHAR_HOR_DOWN_L,
        _ => incoming,
    }
}

fn process_ver_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    // TODO: Handle the transform to corner case
    match incoming {
        CHAR_HOR_L => {
            return if let Some(start_direction) = start_direction {
                match start_direction {
                    LineDirection::LeftToRight => CHAR_VER_RIGHT_L,
                    LineDirection::RightToLeft => CHAR_VER_LEFT_L,
                    _ => unreachable!(),
                }
            } else {
                CHAR_CROSS
            };
        }
        CHAR_CORNER_BL_L | CHAR_CORNER_TL_L => CHAR_VER_RIGHT_L,
        CHAR_CORNER_BR_L | CHAR_CORNER_TR_L => CHAR_VER_LEFT_L,
        _ => incoming,
    }
}

fn process_corner_tl_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(LineDirection::LeftToRight) = start_direction {
                return CHAR_CORNER_TL_L;
            }
            return CHAR_HOR_DOWN_L;
        }
        CHAR_VER_L => {
            if let Some(LineDirection::UpToDown) = start_direction {
                return CHAR_CORNER_TL_L;
            }
            return CHAR_VER_RIGHT_L;
        }
        CHAR_CORNER_TR_L => CHAR_HOR_DOWN_L,
        CHAR_CORNER_BR_L => CHAR_CROSS,
        CHAR_CORNER_BL_L => CHAR_VER_RIGHT_L,
        _ => incoming,
    }
}

fn process_corner_bl_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(LineDirection::LeftToRight) = start_direction {
                return CHAR_CORNER_BL_L;
            }
            return CHAR_HOR_UP_L;
        }
        CHAR_VER_L => {
            if let Some(LineDirection::DownToUp) = start_direction {
                return CHAR_CORNER_BL_L;
            }
            return CHAR_VER_RIGHT_L;
        }
        CHAR_CORNER_BR_L => CHAR_HOR_UP_L,
        CHAR_CORNER_TR_L => CHAR_CROSS,
        CHAR_CORNER_TL_L => CHAR_VER_RIGHT_L,
        _ => incoming,
    }
}

fn process_corner_tr_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(LineDirection::RightToLeft) = start_direction {
                return CHAR_CORNER_TR_L;
            }
            return CHAR_HOR_DOWN_L;
        }
        CHAR_VER_L => {
            if let Some(LineDirection::UpToDown) = start_direction {
                return CHAR_CORNER_TR_L;
            }
            return CHAR_VER_LEFT_L;
        }
        CHAR_CORNER_TL_L => CHAR_HOR_DOWN_L,
        CHAR_CORNER_BL_L => CHAR_CROSS,
        CHAR_CORNER_BR_L => CHAR_VER_LEFT_L,
        _ => incoming,
    }
}

fn process_corner_br_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(LineDirection::RightToLeft) = start_direction {
                return CHAR_CORNER_BR_L;
            }
            return CHAR_HOR_UP_L;
        }
        CHAR_VER_L => {
            if let Some(LineDirection::DownToUp) = start_direction {
                return CHAR_CORNER_BR_L;
            }
            return CHAR_VER_LEFT_L;
        }
        CHAR_CORNER_BL_L => CHAR_HOR_UP_L,
        CHAR_CORNER_TL_L => CHAR_CROSS,
        CHAR_CORNER_TR_L => CHAR_VER_LEFT_L,
        _ => incoming,
    }
}

fn process_hor_up_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_VER_L => {
            if let Some(LineDirection::DownToUp) = start_direction {
                return CHAR_HOR_UP_L;
            }
            return CHAR_CROSS;
        }
        CHAR_HOR_L => CHAR_HOR_UP_L,
        CHAR_CORNER_TL_L | CHAR_CORNER_TR_L => CHAR_CROSS,
        CHAR_CORNER_BL_L | CHAR_CORNER_BR_L => CHAR_HOR_UP_L,
        _ => incoming,
    }
}

fn process_hor_down_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_VER_L => {
            if let Some(LineDirection::UpToDown) = start_direction {
                return CHAR_HOR_DOWN_L;
            }
            return CHAR_CROSS;
        }
        CHAR_HOR_L => CHAR_HOR_DOWN_L,
        CHAR_CORNER_TL_L | CHAR_CORNER_TR_L => CHAR_HOR_DOWN_L,
        CHAR_CORNER_BL_L | CHAR_CORNER_BR_L => CHAR_CROSS,
        _ => incoming,
    }
}

fn process_ver_right_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(LineDirection::LeftToRight) = start_direction {
                return CHAR_VER_RIGHT_L;
            }
            return CHAR_CROSS;
        }
        CHAR_VER_L => CHAR_VER_RIGHT_L,
        CHAR_CORNER_BL_L | CHAR_CORNER_TL_L => CHAR_VER_RIGHT_L,
        CHAR_CORNER_BR_L | CHAR_CORNER_TR_L => CHAR_CROSS,
        _ => incoming,
    }
}

fn process_ver_left_l(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(LineDirection::RightToLeft) = start_direction {
                return CHAR_VER_LEFT_L;
            }
            return CHAR_CROSS;
        }
        CHAR_VER_L => CHAR_VER_LEFT_L,
        CHAR_CORNER_BL_L | CHAR_CORNER_TL_L => CHAR_CROSS,
        CHAR_CORNER_BR_L | CHAR_CORNER_TR_L => CHAR_VER_LEFT_L,
        _ => incoming,
    }
}

fn process_arrow_down(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(direction) = start_direction {
                return match direction {
                    LineDirection::RightToLeft => CHAR_CORNER_BR_L,
                    LineDirection::LeftToRight => CHAR_CORNER_BL_L,
                    _ => unreachable!(),
                };
            }
            incoming
        }
        _ => incoming,
    }
}

fn process_arrow_up(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L => {
            if let Some(direction) = start_direction {
                return match direction {
                    LineDirection::RightToLeft => CHAR_CORNER_TR_L,
                    LineDirection::LeftToRight => CHAR_CORNER_TL_L,
                    _ => unreachable!(),
                };
            }
            incoming
        }
        _ => incoming,
    }
}

fn process_arrow_right(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_VER_L => {
            if let Some(direction) = start_direction {
                return match direction {
                    LineDirection::UpToDown => CHAR_CORNER_TR_L,
                    LineDirection::DownToUp => CHAR_CORNER_BR_L,
                    _ => unreachable!(),
                };
            }
            incoming
        }
        _ => incoming,
    }
}

fn process_arrow_left(start_direction: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_VER_L => {
            if let Some(direction) = start_direction {
                return match direction {
                    LineDirection::UpToDown => CHAR_CORNER_TL_L,
                    LineDirection::DownToUp => CHAR_CORNER_BL_L,
                    _ => unreachable!(),
                };
            }
            incoming
        }
        _ => incoming,
    }
}

fn process_cross(_: Option<LineDirection>, incoming: char) -> char {
    match incoming {
        CHAR_HOR_L | CHAR_VER_L | CHAR_CORNER_TL_L | CHAR_CORNER_TR_L | CHAR_CORNER_BL_L
        | CHAR_CORNER_BR_L => CHAR_CROSS,
        _ => incoming,
    }
}

pub fn calculate_cell_content(
    start_direction: Option<LineDirection>,
    current: char,
    incoming: char,
) -> char {
    // Is overlapping character or not?
    if !current.eq(&CHAR_SPACE) && !current.eq(&CHAR_NEWLINE) {
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
