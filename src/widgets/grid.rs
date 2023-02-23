use std::{fs::File, io::Write, usize};

use druid::{
    commands::{self, NEW_FILE},
    kurbo::Line,
    piet::Text,
    theme, Application, Code, Color, Cursor, Event, FontDescriptor, FontFamily, FontWeight,
    LifeCycleCtx, Point, Rect, RenderContext, Size, TextLayout, Widget,
};

use crate::{
    consts::{CANVAS_SIZE, SELECTION_END_COMMAND, SELECTION_MOVE_COMMAND, SELECTION_START_COMMAND},
    data::{
        grid_list::GridList, selection::SelectionRange, shape_list::ShapeList, ApplicationState,
    },
    tools::{DrawingTools, ToolControl, ToolManager},
};

pub const FONT: &[u8] = include_bytes!("../../assets/iosevka-mono-regular.ttf");

pub struct CanvasGrid {
    width: f64,
    height: f64,
    grid_list: GridList,
    shape_list: ShapeList,
    cell_size: Option<(f64, f64)>,
    letterbox: TextLayout<String>,
    grid_text: TextLayout<String>,
    mouse_position: (usize, usize),
    selection_range: SelectionRange,
    is_mouse_down: bool,
    tool_manager: ToolManager,
}
impl CanvasGrid {
    pub fn new(ctx: &mut LifeCycleCtx) -> Self {
        let monospace_font = ctx.text().load_font(FONT).unwrap_or(FontFamily::MONOSPACE);
        let font = FontDescriptor::new(monospace_font.clone())
            .with_weight(FontWeight::REGULAR)
            .with_size(16.0);
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
            selection_range: SelectionRange::new(),
            letterbox,
            grid_text,
        }
    }

    fn init_grid(&mut self) {
        if let Some((cell_width, cell_height)) = self.cell_size {
            let rows = (self.height / cell_height) as usize;
            let cols = (self.width / cell_width) as usize;
            self.grid_list = GridList::new(cell_width, cell_height, rows, cols);
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
                    keycode => {
                        if data.mode != DrawingTools::Text {
                            // Only handle shortcut key if not in text mode
                            match keycode {
                                Code::Digit1 | Code::KeyL | Code::KeyA => {
                                    data.mode = DrawingTools::Line;
                                }
                                Code::Digit2 | Code::KeyR => {
                                    data.mode = DrawingTools::Rect;
                                }
                                Code::Digit3 | Code::KeyT => {
                                    data.mode = DrawingTools::Text;
                                }
                                Code::Digit4 | Code::KeyE => {
                                    data.mode = DrawingTools::Eraser;
                                }
                                _ => {}
                            }

                            if event.mods.meta() || event.mods.ctrl() {
                                match keycode {
                                    Code::KeyC => {
                                        // copy current diagram to clipboard
                                        Application::global()
                                            .clipboard()
                                            .put_string(self.grid_list.to_string());
                                    }
                                    Code::KeyN => {
                                        ctx.submit_command(NEW_FILE);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                self.tool_manager
                    .input(ctx, event, &mut self.shape_list, &mut self.grid_list);
                ctx.request_update();
            }
            Event::MouseMove(event) => {
                if let Some((cell_width, cell_height)) = self.cell_size {
                    let mouse_row = (event.pos.y / cell_height) as usize;
                    let mouse_col = (event.pos.x / cell_width) as usize;
                    self.mouse_position = (mouse_row, mouse_col);
                    if self.is_mouse_down {
                        self.tool_manager.draw(
                            ctx,
                            event,
                            &mut self.shape_list,
                            &mut self.grid_list,
                        );
                    }
                    ctx.request_update();
                }
            }
            Event::MouseDown(event) => {
                self.is_mouse_down = true;
                self.tool_manager
                    .start(ctx, event, &mut self.shape_list, &mut self.grid_list);
            }
            Event::MouseUp(event) => {
                self.is_mouse_down = false;
                self.tool_manager
                    .end(ctx, event, &mut self.shape_list, &mut self.grid_list);
                self.shape_list.commit(&mut self.grid_list);
                ctx.request_update();
            }
            Event::Command(cmd) => {
                if let Some(point) = cmd.get(SELECTION_START_COMMAND) {
                    self.selection_range.set_start(*point);
                }
                if let Some(point) = cmd.get(SELECTION_MOVE_COMMAND) {
                    self.selection_range.set_end(*point);
                }
                if let Some(point) = cmd.get(SELECTION_END_COMMAND) {
                    if let Some(rect) = self.selection_range.as_rect() {
                        // Selected a range
                        let matched = self
                            .shape_list
                            .find_shape_in_rect(rect, &mut self.grid_list);

                        println!(
                            "FOUND {:?}",
                            matched.iter().map(|s| s.get_points()).collect::<Vec<_>>()
                        );
                    } else {
                        // Selected a single point
                        if let Some(matched) = self
                            .shape_list
                            .find_shape_in_point(*point, &mut self.grid_list)
                        {
                            println!("SELECT SINGLE POINT {:?}", matched.get_points());
                        }
                    }
                    // TODO: Visually highlight selected shapes, and make them movable
                    self.selection_range.discard();
                }
                if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
                    println!("Save File {:?}", file_info.path());
                    if let Ok(mut file) = File::create(file_info.path()) {
                        _ = file.write_all(self.grid_list.to_string().as_bytes());
                        if let Some(file_name) =
                            file_info.path().to_str().and_then(|s| Some(s.to_string()))
                        {
                            data.current_file = Some(file_name.clone());
                            ctx.window().set_title(file_name.as_str());
                        }
                    }
                }
                if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
                    match std::fs::read_to_string(file_info.path()) {
                        Ok(content) => {
                            self.grid_list.clear_all();
                            self.grid_list.load_content(content);
                            if let Some(file_name) =
                                file_info.path().to_str().and_then(|s| Some(s.to_string()))
                            {
                                data.current_file = Some(file_name.clone());
                                ctx.window().set_title(file_name.as_str());
                            }
                        }
                        Err(e) => {
                            println!("Error opening file: {e}");
                        }
                    }
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
                self.grid_list.clear_all_highlight();
            }

            match data.mode {
                DrawingTools::Select => ctx.set_cursor(&Cursor::Arrow),
                DrawingTools::Line => ctx.set_cursor(&Cursor::Crosshair),
                DrawingTools::Rect => ctx.set_cursor(&Cursor::Crosshair),
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
            let primary_color = env.get(theme::PRIMARY_LIGHT);
            let selection_brush = ctx.solid_brush(primary_color.with_alpha(0.5));

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
                let _rows = (self.height / cell_height) as usize;

                for row in (start.1)..=(end.1) {
                    let row = row as f64;
                    let line = Line::new(
                        Point::new(bound.x0, row * cell_height),
                        Point::new(bound.x1, row * cell_height),
                    );
                    ctx.stroke(line, &grid_brush, 1.0);
                }
                for col in (start.0)..=(end.0) {
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

                if let Some(rect) = self.selection_range.as_rect() {
                    ctx.fill(rect, &selection_brush);
                }
            }
        });
    }
}
