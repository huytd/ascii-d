use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, widget::Scroll};

mod state;
mod widgets;
use state::{ApplicationState, OperationMode};
use widgets::{grid::CanvasGrid, layout::StackLayout, status_label::StatusLabel};

fn build_ui() -> impl Widget<ApplicationState> {
    let mut ui = StackLayout::new();
    ui.add_child(Scroll::new(CanvasGrid::new()));
    ui.add_child(StatusLabel::new());
    ui
}

fn main() -> Result<(), PlatformError> {
    // https://github.com/linebender/druid/pull/1701/files
    // Follow the above PR for transparent title bar status
    let app =
    AppLauncher::with_window(
        WindowDesc::new(build_ui())
    );
    app.launch(ApplicationState { mode: OperationMode::Normal })?;
    Ok(())
}
