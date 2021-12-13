use druid::{widget::Scroll, AppLauncher, PlatformError, Widget, WindowDesc};

mod consts;
mod data;
mod shapes;
mod tools;
mod widgets;

use data::ApplicationState;
use widgets::{grid::CanvasGrid, layout::StackLayout, toolbar::ToolBarWidget};

fn build_ui() -> impl Widget<ApplicationState> {
    let mut ui = StackLayout::new();
    ui.add_child(Scroll::new(CanvasGrid::new()));
    ui.add_child(ToolBarWidget::new());
    ui
}

fn main() -> Result<(), PlatformError> {
    // https://github.com/linebender/druid/pull/1701/files
    // Follow the above PR for transparent title bar status
    let app = AppLauncher::with_window(WindowDesc::new(build_ui()).title("ASCII-d"));
    app.launch(ApplicationState {
        mode: String::new(),
    })?;
    Ok(())
}
