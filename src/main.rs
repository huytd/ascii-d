use druid::{AppLauncher, Color, Data, FontDescriptor, FontFamily, PlatformError, Point, Rect, RenderContext, Size, TextLayout, Widget, WidgetExt, WidgetPod, WindowConfig, WindowDesc, kurbo::Line, widget::{self, Button, Container, Flex, Label, LabelText, Scroll, SizedBox}};

struct ArenaWidget<T> where T: Data {
    children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
}

impl<T: Data> ArenaWidget<T> {
    pub fn new() -> Self {
        ArenaWidget { children: vec![] }
    }

    pub fn add_child(&mut self, w: impl Widget<T> + 'static) {
        self.children.push(WidgetPod::new(Box::new(w)));
    }
}

impl<T: Data> Widget<T> for ArenaWidget<T> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &druid::Env) {
        for child in self.children.iter_mut().filter_map(|x| Some(x.widget_mut())) {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &T, env: &druid::Env) {
        for child in self.children.iter_mut().filter_map(|x| Some(x.widget_mut())) {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, data: &T, env: &druid::Env) {
        for child in self.children.iter_mut().filter_map(|x| Some(x.widget_mut())) {
            child.update(ctx, _old_data, data, env)
        }
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &druid::Env) -> druid::Size {
        for child in self.children.iter_mut().filter_map(|x| Some(x.widget_mut())) {
            child.layout(ctx, bc, data, env);
        }
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        for child in self.children.iter_mut().filter_map(|x| Some(x.widget_mut())) {
            child.paint(ctx, data, env);
        }
    }
}

struct RoudedLabel {
    text: String
}
impl RoudedLabel {
    pub fn new(text: &str) -> Self {
        RoudedLabel { text: text.to_string() }
    }
}
impl Widget<()> for RoudedLabel {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut (), env: &druid::Env) { }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &(), env: &druid::Env) { }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &(), data: &(), env: &druid::Env) { }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &(), env: &druid::Env) -> Size {
        Size { width: 100.0, height: 20.0 }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &(), env: &druid::Env) {
        let size = ctx.size();
        let brush = ctx.solid_brush(Color::WHITE);
        let rect = Rect::new(20.0, size.height - 50.0, 100.0, size.height - 20.0).to_rounded_rect(5.0);
        ctx.fill(rect, &brush);
    }
}

struct CanvasGrid {
    width: f64,
    height: f64,
    cell_size: Option<(f64, f64)>,
    letterbox: TextLayout<String>
}
impl CanvasGrid {
    pub fn new() -> Self {
        let font = FontDescriptor::new(FontFamily::MONOSPACE).with_size(16.0);
        let mut letterbox = TextLayout::<String>::new();
        letterbox.set_font(font);
        letterbox.set_text("H".to_string());
        CanvasGrid {
            width: 5000.0,
            height: 5000.0,
            cell_size: None,
            letterbox
        }
    }
}
impl Widget<()> for CanvasGrid {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut (), env: &druid::Env) { }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &(), env: &druid::Env) { }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &(), data: &(), env: &druid::Env) { }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &(), env: &druid::Env) -> Size {
        self.letterbox.rebuild_if_needed(ctx.text(), env);
        Size { width: self.width, height: self.height }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &(), env: &druid::Env) {
        if self.cell_size.is_none() {
            let lsize = self.letterbox.size();
            self.cell_size = Some((lsize.width, lsize.height));
        }

        let size = ctx.size();
        let brush = ctx.solid_brush(Color::BLACK);
        ctx.fill(size.to_rect(), &brush);

        let grid_brush = ctx.solid_brush(Color::WHITE.with_alpha(0.1));
        if let Some((cell_width, cell_height)) = self.cell_size {
            let rows = (self.height / cell_height) as u32;
            let cols = (self.width / cell_width) as u32;
            for row in 0..rows {
                let row = row as f64;
                let line = Line::new(Point::new(0.0, row * cell_height), Point::new(size.width, row * cell_height));
                ctx.stroke(line, &grid_brush, 1.0);
            }
            for col in 0..cols {
                let col = col as f64;
                let line = Line::new(Point::new(col * cell_width, 0.0), Point::new(col * cell_width, size.height));
                ctx.stroke(line, &grid_brush, 1.0);
            }
        }
    }
}

fn build_ui() -> impl Widget<()> {
    let mut ui: ArenaWidget<()> = ArenaWidget::new();
    ui.add_child(Scroll::new(CanvasGrid::new()));
    ui.add_child(RoudedLabel::new("NORMAL"));
    ui
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(
        WindowDesc::new(build_ui())
    ).launch(())?;
    Ok(())
}
