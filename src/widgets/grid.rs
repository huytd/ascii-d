use std::usize;

use druid::{
    kurbo::Line, Code, Color, Cursor, Event, FontDescriptor, FontFamily, Point, Rect,
    RenderContext, Size, TextLayout, Widget,
};

use crate::{
    consts::CANVAS_SIZE,
    data::{ApplicationState, GridCell, GridList},
    shapes::ShapeList,
    tools::{DrawingTools, ToolControl, ToolManager},
};

pub struct CanvasGrid {
    width: f64,
    height: f64,
    grid_list: GridList,
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
            grid_list: GridList::default(),
            shape_list: ShapeList::new(),
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
            let rows = (self.height / cell_height) as usize;
            let cols = (self.width / cell_width) as usize;
            self.grid_list = GridList::new(cell_width, cell_height, rows, cols);
            for row in 0..rows {
                for col in 0..cols {
                    let i = row * cols + col;
                    if i >= cols && i % cols == 0 {
                        let cell = self.grid_list.get(i as usize);
                        *cell = GridCell::newline();
                    }
                }
            }
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
                ctx.request_focus();
            }
            Event::KeyDown(event) => {
                match event.code {
                    Code::Escape => {
                        data.mode = DrawingTools::Select;
                    }
                    _ => {}
                }

                if let Some((cell_width, cell_height)) = self.cell_size {
                    self.tool_manager
                        .input(event, &mut self.shape_list, &mut self.grid_list);
                }
                ctx.request_update();
            }
            Event::MouseMove(event) => {
                if let Some((cell_width, cell_height)) = self.cell_size {
                    let mouse_row = (event.pos.y / cell_height) as usize;
                    let mouse_col = (event.pos.x / cell_width) as usize;
                    self.mouse_position = (mouse_row, mouse_col);

                    if self.is_mouse_down {
                        self.tool_manager
                            .draw(event, &mut self.shape_list, &mut self.grid_list);
                    }
                    ctx.request_update();
                }
            }
            Event::MouseDown(event) => {
                self.is_mouse_down = true;
                if let Some((cell_width, cell_height)) = self.cell_size {
                    self.tool_manager
                        .start(event, &mut self.shape_list, &mut self.grid_list);
                }
            }
            Event::MouseUp(event) => {
                self.is_mouse_down = false;

                if let Some((cell_width, cell_height)) = self.cell_size {
                    self.tool_manager
                        .end(event, &mut self.shape_list, &mut self.grid_list);
                    self.shape_list.commit(&mut self.grid_list);
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
        old_data: &ApplicationState,
        data: &ApplicationState,
        _env: &druid::Env,
    ) {
        if old_data.mode != data.mode {
            self.tool_manager.set_tool(data.mode);
            if old_data.mode == DrawingTools::Text {
                self.shape_list.commit_all(&mut self.grid_list);
                self.grid_list.clear_highlight_all();
            }

            match data.mode {
                DrawingTools::Select => ctx.set_cursor(&Cursor::Arrow),
                DrawingTools::Line => ctx.set_cursor(&Cursor::Crosshair),
                DrawingTools::Text => ctx.set_cursor(&Cursor::IBeam),
                DrawingTools::Eraser => ctx.set_cursor(&Cursor::Crosshair),
            }
        }
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
        let brush = ctx.solid_brush(Color::WHITE);
        ctx.with_save(|ctx| {
            ctx.clip(bound);
            ctx.fill(bound, &brush);
            let grid_brush = ctx.solid_brush(Color::rgb(0.91, 0.91, 0.91));
            let cursor_brush = ctx.solid_brush(Color::RED);

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

                self.shape_list.draw(&mut self.grid_list);

                for row in (start.1)..(end.1) {
                    for col in (start.0)..(end.0) {
                        let i = row * cols + col;

                        if self.grid_list.get(i).highlighted {
                            let h_row = row as f64;
                            let h_col = col as f64;
                            let h_rect = Rect::new(
                                h_col * cell_width,
                                h_row * cell_height,
                                h_col * cell_width + cell_width,
                                h_row * cell_height + cell_height,
                            );
                            ctx.stroke(h_rect, &cursor_brush, 1.0);
                        }

                        let cell_content = self.grid_list.get(i).read();
                        if !cell_content.is_ascii_whitespace() {
                            self.grid_text.set_text(cell_content.to_string());
                            if self.grid_list.get(i).preview.is_some() {
                                self.grid_text.set_text_color(Color::RED);
                            } else {
                                self.grid_text.set_text_color(Color::BLACK);
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
