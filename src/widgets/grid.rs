use druid::{Code, Color, Event, FontDescriptor, FontFamily, Point, RenderContext, Size, TextLayout, Widget, kurbo::Line};

use crate::state::{ApplicationState, OperationMode};

pub struct CanvasGrid {
    width: f64,
    height: f64,
    cell_size: Option<(f64, f64)>,
    letterbox: TextLayout<String>,
    mouse_position: Point
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
            letterbox,
            mouse_position: Point::ORIGIN
        }
    }
}
impl Widget<ApplicationState> for CanvasGrid {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut ApplicationState, _env: &druid::Env) {
        match event {
            Event::WindowConnected => {
                // Have to request focus in order to get keyboard event
                ctx.request_focus();
            },
            Event::KeyDown(event) => {
                match event.code {
                    Code::Digit1 => data.mode = OperationMode::Draw,
                    Code::Digit2 => data.mode = OperationMode::Text,
                    Code::Digit3 => data.mode = OperationMode::Erase,
                    Code::Digit4 => data.mode = OperationMode::Visual,
                    Code::Escape => data.mode = OperationMode::Normal,
                    _ => {}
                }
                ctx.request_update();
            },
            Event::MouseMove(event) => {
                self.mouse_position = event.pos;
                ctx.request_paint();
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &ApplicationState, _env: &druid::Env) { }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &ApplicationState, _data: &ApplicationState, _env: &druid::Env) { }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &ApplicationState, env: &druid::Env) -> Size {
        self.letterbox.rebuild_if_needed(ctx.text(), env);
        Size { width: self.width, height: self.height }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &ApplicationState, _env: &druid::Env) {
        if self.cell_size.is_none() {
            let lsize = self.letterbox.size();
            self.cell_size = Some((lsize.width, lsize.height));
        }

        let size = ctx.size();
        let brush = ctx.solid_brush(Color::BLACK);
        ctx.fill(size.to_rect(), &brush);

        let cursor_brush = ctx.solid_brush(Color::YELLOW);
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

            let mouse_row = (self.mouse_position.y / cell_height) as u32;
            let mouse_col = (self.mouse_position.x / cell_width) as u32;
            println!("POS {} {}", mouse_row, mouse_col);
        }
    }
}
