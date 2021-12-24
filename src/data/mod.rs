use druid::Data;

use crate::tools::DrawingTools;

pub mod grid_cell;
pub mod grid_list;
pub mod selection;

#[derive(Clone, PartialEq, Data)]
pub struct ApplicationState {
    pub mode: DrawingTools,
}
