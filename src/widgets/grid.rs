use std::usize;

use druid::{Code, Color, Event, FontDescriptor, FontFamily, Point, Rect, RenderContext, Size, TextLayout, Widget, kurbo::Line, widget::Label};

use crate::state::{ApplicationState, OperationMode};

pub struct CanvasGrid {
    width: f64,
    height: f64,
    data: Vec<char>,
    cell_size: Option<(f64, f64)>,
    letterbox: TextLayout<String>,
    mouse_position: (usize, usize),
    is_mouse_down: bool,
    grid_label: TextLayout<String>
}
impl CanvasGrid {
    pub fn new() -> Self {
        let font = FontDescriptor::new(FontFamily::MONOSPACE).with_size(16.0);
        let mut letterbox = TextLayout::<String>::new();
        letterbox.set_font(font.clone());
        letterbox.set_text("H".to_string());
        let size = 500.0;
        let mut grid_label = TextLayout::<String>::new();
        grid_label.set_font(font.clone());
        grid_label.set_text_color(Color::WHITE);
        CanvasGrid {
            width: size,
            height: size,
            data: vec![' '; size as usize * size as usize],
            cell_size: None,
            mouse_position: (0, 0),
            is_mouse_down: false,
            letterbox,
            grid_label
        }
    }

    fn set_char_at(&mut self, at: (usize, usize), c: char) {
        let i = at.0 * self.width as usize + at.1;
        self.data[i] = c;
        self.update_grid_label();
    }

    fn update_grid_label(&mut self,) {
        let text: String = self.data.chunks(self.width as usize).map(|s| s.to_vec()).collect::<Vec<_>>().join(&'\n').iter().collect();
        self.grid_label.set_text(text);
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
                if let Some((cell_width, cell_height)) = self.cell_size {
                    let mouse_row = (event.pos.y / cell_height) as usize;
                    let mouse_col = (event.pos.x / cell_width) as usize;
                    self.mouse_position = (mouse_row, mouse_col);
                }

                if self.is_mouse_down {
                    self.set_char_at(self.mouse_position, '+');
                }

                ctx.request_paint();
            },
            Event::MouseDown(_) => {
                self.is_mouse_down = true;
            },
            Event::MouseUp(_) => {
                self.is_mouse_down = false;
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, _data: &ApplicationState, _env: &druid::Env) {
        match event {
            druid::LifeCycle::WidgetAdded => self.update_grid_label(),
            _ => {}
        }
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &ApplicationState, _data: &ApplicationState, _env: &druid::Env) {
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &ApplicationState, env: &druid::Env) -> Size {
        self.letterbox.rebuild_if_needed(ctx.text(), env);
        self.grid_label.rebuild_if_needed(ctx.text(), env);
        Size { width: self.width, height: self.height }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &ApplicationState, env: &druid::Env) {
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
            let rows = (self.height / cell_height) as u64;
            let cols = (self.width / cell_width) as u64;
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

            let mouse_row = self.mouse_position.0 as f64;
            let mouse_col = self.mouse_position.1 as f64;
            let cursor_rect = Rect::new(
                mouse_col * cell_width, mouse_row * cell_height,
                mouse_col * cell_width + cell_width, mouse_row * cell_height + cell_height);
            ctx.fill(cursor_rect, &cursor_brush);

            self.grid_label.rebuild_if_needed(ctx.text(), env);
            self.grid_label.draw(ctx, Point::ORIGIN);
        }
    }
}
