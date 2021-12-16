use consts::FONT_KEY;
use druid::{
    piet::Text,
    widget::{Scroll, SizedBox, WidgetExt},
    AppLauncher, FontDescriptor, FontWeight, Key, LifeCycle, PlatformError, Widget, WidgetPod,
    WindowDesc,
};

mod consts;
mod data;
mod shapes;
mod tools;
mod widgets;

use data::ApplicationState;
use widgets::{grid::CanvasGrid, layout::StackLayout, toolbar::ToolBarWidget};

pub const FONT: &[u8] = include_bytes!("../assets/iosevka-mono-regular.ttf");

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
            let monospace_font = ctx.text().load_font(FONT).unwrap();
            let mut ui = StackLayout::new();
            ui.add_child(Scroll::new(CanvasGrid::new()));
            ui.add_child(ToolBarWidget::new());
            self.content = WidgetPod::new(Box::new(ui.env_scope(move |env, _data| {
                let mono_regular =
                    FontDescriptor::new(monospace_font.clone()).with_weight(FontWeight::REGULAR);

                env.set(FONT_KEY, mono_regular.clone());
            })));
        }

        self.content.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &ApplicationState,
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

fn main() -> Result<(), PlatformError> {
    // https://github.com/linebender/druid/pull/1701/files
    // Follow the above PR for transparent title bar status
    let app = AppLauncher::with_window(WindowDesc::new(MainWindow::new()).title("ASCII-d"));
    app.launch(ApplicationState {
        mode: tools::DrawingTools::Select,
    })?;
    Ok(())
}
