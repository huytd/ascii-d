use std::fmt::Display;

use druid::Data;

#[derive(Clone, PartialEq, Data)]
pub struct ApplicationState {
    pub mode: String,
}
