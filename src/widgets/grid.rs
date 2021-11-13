use std::usize;

use druid::{
    kurbo::Line, Code, Color, Event, FontDescriptor, FontFamily, Point, Rect, RenderContext, Size,
    TextLayout, Widget,
};

use crate::{
    consts::CANVAS_SIZE,
    data::{ApplicationState, GridCell},
    shapes::ShapeList,
    tools::{DrawingTools, ToolControl, ToolManager},
};

pub struct CanvasGrid {
    width: f64,
    height: f64,
    data: Vec<GridCell>,
    shape_list: ShapeList,
    cell_size: Option<(f64, f64)>,
    letterbox: TextLayout<String>,
    grid_text: TextLayout<String>,
    mouse_position: (usize, usize),
    is_mouse_down: bool,
    tool_manager: ToolManager,
}
impl CanvasGrid {
    pub fn new() -> Self {
        let font = FontDescriptor::new(FontFamily::MONOSPACE).with_size(16.0);
        let mut letterbox = TextLayout::<String>::new();
        letterbox.set_font(font.clone());
        letterbox.set_text("H".to_string());
        let mut grid_text = TextLayout::<String>::new();
        grid_text.set_font(font.clone());
        grid_text.set_text("+".to_string());
        CanvasGrid {
            width: CANVAS_SIZE,
            height: CANVAS_SIZE,
            data: Vec::new(),
            shape_list: vec![],
            cell_size: None,
            mouse_position: (0, 0),
            is_mouse_down: false,
            tool_manager: ToolManager::new(),
            letterbox,
            grid_text,
        }
    }

    fn init_grid(&mut self) {
        if let Some((cell_width, cell_height)) = self.cell_size {
            let rows = (self.height / cell_height) as u64;
            let cols = (self.width / cell_width) as u64;
            self.data = vec![GridCell::empty(); (rows * cols) as usize];
            for row in 0..rows {
                for col in 0..cols {
                    let i = row * cols + col;
                    if i >= cols && i % cols == 0 {
                        self.data[i as usize] = GridCell::newline();
                    }
                }
            }
            println!("INIT GRID");
        }
    }
}
impl Widget<ApplicationState> for CanvasGrid {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut ApplicationState,
        _env: &druid::Env,
    ) {
        match event {
            Event::WindowConnected => {
                // Have to request focus in order to get keyboard event
                data.mode = self.tool_manager.get_active_tool().to_string();
                ctx.request_focus();
            }
            Event::KeyDown(event) => {
                match event.code {
                    Code::Digit1 => self.tool_manager.set_tool(DrawingTools::Line),
                    Code::Digit2 => self.tool_manager.set_tool(DrawingTools::Text),
                    _ => {}
                }
                data.mode = self.tool_manager.get_active_tool().to_string();
                if let Some((cell_width, cell_height)) = self.cell_size {
                    let rows = (self.height / cell_height) as usize;
                    let cols = (self.width / cell_width) as usize;
                    self.tool_manager.input(
                        event,
                        &mut self.shape_list,
                        (cell_width, cell_height),
                        (rows, cols),
                    );
                }
                ctx.request_update();
            }
            Event::MouseMove(event) => {
                if let Some((cell_width, cell_height)) = self.cell_size {
                    let rows = (self.height / cell_height) as usize;
                    let cols = (self.width / cell_width) as usize;
                    let mouse_row = (event.pos.y / cell_height) as usize;
                    let mouse_col = (event.pos.x / cell_width) as usize;
                    self.mouse_position = (mouse_row, mouse_col);

                    if self.is_mouse_down {
                        self.tool_manager.draw(
                            event,
                            &mut self.shape_list,
                            (cell_width, cell_height),
                            (rows, cols),
                        );
                    }
                    ctx.request_update();
                }
            }
            Event::MouseDown(event) => {
                self.is_mouse_down = true;
                if let Some((cell_width, cell_height)) = self.cell_size {
                    let rows = (self.height / cell_height) as usize;
                    let cols = (self.width / cell_width) as usize;
                    self.tool_manager.start(
                        event,
                        &mut self.shape_list,
                        (cell_width, cell_height),
                        (rows, cols),
                    );
                }
            }
            Event::MouseUp(event) => {
                self.is_mouse_down = false;

                if let Some((cell_width, cell_height)) = self.cell_size {
                    let rows = (self.height / cell_height) as usize;
                    let cols = (self.width / cell_width) as usize;
                    self.tool_manager.end(
                        event,
                        &mut self.shape_list,
                        (cell_width, cell_height),
                        (rows, cols),
                    );
                    for shape in self.shape_list.iter_mut() {
                        if shape.is_preview() && !shape.is_manual_commit() {
                            shape.commit(&mut self.data);
                        }
                    }
                    ctx.request_update();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        _data: &ApplicationState,
        _env: &druid::Env,
    ) {
        match event {
            druid::LifeCycle::WidgetAdded => {}
            _ => {}
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &ApplicationState,
        _data: &ApplicationState,
        _env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &ApplicationState,
        env: &druid::Env,
    ) -> Size {
        if self.cell_size.is_none() {
            self.letterbox.rebuild_if_needed(ctx.text(), env);
            let lsize = self.letterbox.size();
            self.cell_size = Some((lsize.width, lsize.height));
            self.init_grid();
        }
        self.grid_text.rebuild_if_needed(ctx.text(), env);
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &ApplicationState, env: &druid::Env) {
        let bound = ctx.region().bounding_box();
        let brush = ctx.solid_brush(Color::rgb(0.08, 0.08, 0.08));
        ctx.with_save(|ctx| {
            ctx.clip(bound);
            ctx.fill(bound, &brush);
            let grid_brush = ctx.solid_brush(Color::BLACK);
            let cursor_brush = ctx.solid_brush(Color::YELLOW);

            if let Some((cell_width, cell_height)) = self.cell_size {
                let start = (
                    (bound.x0 / cell_width) as usize,
                    (bound.y0 / cell_height) as usize,
                );
                let end = (
                    (bound.x1 / cell_width) as usize,
                    (bound.y1 / cell_height) as usize,
                );
                let cols = (self.width / cell_width) as usize;
                let rows = (self.height / cell_height) as usize;

                for row in (start.1)..(end.1) {
                    let row = row as f64;
                    let line = Line::new(
                        Point::new(bound.x0, row * cell_height),
                        Point::new(bound.x1, row * cell_height),
                    );
                    ctx.stroke(line, &grid_brush, 1.0);
                }
                for col in (start.0)..(end.0) {
                    let col = col as f64;
                    let line = Line::new(
                        Point::new(col * cell_width, bound.y0),
                        Point::new(col * cell_width, bound.y1),
                    );
                    ctx.stroke(line, &grid_brush, 1.0);
                }

                let mouse_row = self.mouse_position.0 as f64;
                let mouse_col = self.mouse_position.1 as f64;
                let cursor_rect = Rect::new(
                    mouse_col * cell_width,
                    mouse_row * cell_height,
                    mouse_col * cell_width + cell_width,
                    mouse_row * cell_height + cell_height,
                );
                ctx.fill(cursor_rect, &cursor_brush);

                if let Some(shape) = self.shape_list.last_mut() {
                    if shape.is_preview() {
                        shape.draw(&mut self.data, (cell_width, cell_height), (rows, cols));
                    }
                }

                for row in (start.1)..(end.1) {
                    for col in (start.0)..(end.0) {
                        let i = row * cols + col;
                        let cell_content = self.data[i].read();
                        if !cell_content.is_ascii_whitespace() {
                            self.grid_text.set_text(cell_content.to_string());
                            if self.data[i].preview.is_some() {
                                self.grid_text.set_text_color(Color::RED);
                            } else {
                                self.grid_text.set_text_color(Color::WHITE);
                            }
                            self.grid_text.rebuild_if_needed(ctx.text(), env);
                            self.grid_text
                                .draw(ctx, (col as f64 * cell_width, row as f64 * cell_height));
                        }
                    }
                }
            }
        });
    }
}
