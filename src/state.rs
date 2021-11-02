use std::fmt::Display;

use druid::Data;

#[derive(Clone, PartialEq, Data)]
pub enum OperationMode {
    Normal,
    Draw,
    Erase,
    Text,
    Visual,
}

impl Display for OperationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            OperationMode::Normal => "NORMAL",
            OperationMode::Draw => "DRAWING",
            OperationMode::Erase => "ERASE",
            OperationMode::Text => "TEXT INPUT",
            OperationMode::Visual => "VISUAL",
        };
        write!(f, "{}", op)
    }
}

#[derive(Clone, PartialEq, Data)]
pub struct ApplicationState {
    pub mode: OperationMode,
}
