use druid::{Data, WindowId};
use std::collections::HashMap;

use crate::tools::DrawingTools;

pub mod grid_cell;
pub mod grid_list;
pub mod history;
pub mod selection;
pub mod shape_list;

mod overlap;

#[derive(Clone, PartialEq, Data, Debug)]
pub struct WindowData {
    pub mode: DrawingTools,
    pub current_file: Option<String>,
}

impl WindowData {
    pub fn new() -> Self {
        Self {
            mode: DrawingTools::Select,
            current_file: None,
        }
    }
}

#[derive(Clone, PartialEq, Data, Debug)]
pub struct ApplicationState {
    #[data(eq)]
    pub windows: HashMap<WindowId, WindowData>,
}
