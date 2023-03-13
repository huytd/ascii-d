use druid::Data;

use crate::tools::DrawingTools;

pub mod grid_cell;
pub mod grid_list;
pub mod selection;
pub mod shape_list;

mod overlap;

#[derive(Clone, PartialEq, Data)]
pub struct ApplicationState {
    pub mode: DrawingTools,
    pub current_file: Option<String>,
}
