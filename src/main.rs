use druid::{
    commands,
    widget::{Scroll, SizedBox},
    AppDelegate, AppLauncher, Application, Command, DelegateCtx, Env, Event, Handled, LifeCycle,
    PlatformError, Target, Widget, WidgetPod, WindowDesc, WindowId,
};

#[macro_use]
mod macros;
mod consts;
mod data;
mod shapes;
mod tools;
mod widgets;

use data::ApplicationState;
use widgets::{grid::CanvasGrid, layout::StackLayout, toolbar::ToolBarWidget};

struct MainWindow {
    content: WidgetPod<ApplicationState, Box<dyn Widget<ApplicationState>>>,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            content: WidgetPod::new(Box::new(SizedBox::empty())),
        }
    }
}

impl Widget<ApplicationState> for MainWindow {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut ApplicationState,
        env: &druid::Env,
    ) {
        self.content.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &ApplicationState,
        env: &druid::Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            let mut ui = StackLayout::new();
            ui.add_child(Scroll::new(CanvasGrid::new(ctx)));
            ui.add_child(ToolBarWidget::new());
            self.content = WidgetPod::new(Box::new(ui));
        }
        self.content.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &ApplicationState,
        data: &ApplicationState,
        env: &druid::Env,
    ) {
        self.content.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &ApplicationState,
        env: &druid::Env,
    ) -> druid::Size {
        self.content.layout(ctx, bc, data, env);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &ApplicationState, env: &druid::Env) {
        self.content.paint(ctx, data, env);
    }
}

struct Delegate;

impl AppDelegate<ApplicationState> for Delegate {
    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut ApplicationState,
        env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        // Quit when the window is closed
        Application::global().quit();
    }
}

fn main() -> Result<(), PlatformError> {
    // https://github.com/linebender/druid/pull/1701/files
    // Follow the above PR for transparent title bar status
    let app = AppLauncher::with_window(WindowDesc::new(MainWindow::new()).title("ASCII-d"));
    app.delegate(Delegate).launch(ApplicationState {
        mode: tools::DrawingTools::Select,
    })?;
    Ok(())
}
