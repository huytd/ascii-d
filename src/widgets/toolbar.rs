use std::path::PathBuf;

use super::image_button::ImageButton;
use crate::{consts::BUTTON_HIGHLIGHT_COMMAND, data::ApplicationState, tools::DrawingTools};
use druid::{
    widget::{CrossAxisAlignment, Flex, MainAxisAlignment},
    Color, Event, FileDialogOptions, FileInfo, FileSpec, ImageBuf, Point, Rect, RenderContext,
    Size, Widget, WidgetPod,
};

pub struct ToolBarWidget {
    left_buttons: WidgetPod<ApplicationState, Flex<ApplicationState>>,
    right_buttons: WidgetPod<ApplicationState, Flex<ApplicationState>>,
}

impl ToolBarWidget {
    pub fn new() -> Self {
        let select_icon =
            ImageBuf::from_data(include_bytes!("../../assets/select-icon.png")).unwrap();
        let line_icon = ImageBuf::from_data(include_bytes!("../../assets/line-icon.png")).unwrap();
        let text_icon = ImageBuf::from_data(include_bytes!("../../assets/text-icon.png")).unwrap();
        let rect_icon = ImageBuf::from_data(include_bytes!("../../assets/rect-icon.png")).unwrap();
        let eraser_icon =
            ImageBuf::from_data(include_bytes!("../../assets/eraser-icon.png")).unwrap();
        let save_icon = ImageBuf::from_data(include_bytes!("../../assets/save-icon.png")).unwrap();
        let open_icon = ImageBuf::from_data(include_bytes!("../../assets/open-icon.png")).unwrap();

        let left_buttons = WidgetPod::new(
            Flex::row()
                .with_child(
                    ImageButton::new(
                        select_icon,
                        Size::new(26.0, 26.0),
                        DrawingTools::Select.to_string(),
                    )
                    .on_click(|ctx, data: &mut ApplicationState, _env| {
                        let tool = DrawingTools::Select;
                        data.mode = tool;
                        ctx.submit_notification(BUTTON_HIGHLIGHT_COMMAND.with(tool.to_string()));
                        ctx.set_handled();
                    }),
                )
                .with_spacer(4.0)
                .with_child(
                    ImageButton::new(
                        line_icon,
                        Size::new(26.0, 26.0),
                        DrawingTools::Line.to_string(),
                    )
                    .on_click(|ctx, data: &mut ApplicationState, _env| {
                        let tool = DrawingTools::Line;
                        data.mode = tool;
                        ctx.submit_notification(BUTTON_HIGHLIGHT_COMMAND.with(tool.to_string()));
                        ctx.set_handled();
                    }),
                )
                .with_spacer(4.0)
                .with_child(
                    ImageButton::new(
                        rect_icon,
                        Size::new(26.0, 26.0),
                        DrawingTools::Rect.to_string(),
                    )
                    .on_click(|ctx, data: &mut ApplicationState, _env| {
                        let tool = DrawingTools::Rect;
                        data.mode = tool;
                        ctx.submit_notification(BUTTON_HIGHLIGHT_COMMAND.with(tool.to_string()));
                        ctx.set_handled();
                    }),
                )
                .with_spacer(4.0)
                .with_child(
                    ImageButton::new(
                        text_icon,
                        Size::new(26.0, 26.0),
                        DrawingTools::Text.to_string(),
                    )
                    .on_click(|ctx, data: &mut ApplicationState, _env| {
                        let tool = DrawingTools::Text;
                        data.mode = tool;
                        ctx.submit_notification(BUTTON_HIGHLIGHT_COMMAND.with(tool.to_string()));
                        ctx.set_handled();
                    }),
                )
                .with_spacer(4.0)
                .with_child(
                    ImageButton::new(
                        eraser_icon,
                        Size::new(26.0, 26.0),
                        DrawingTools::Eraser.to_string(),
                    )
                    .on_click(|ctx, data: &mut ApplicationState, _env| {
                        let tool = DrawingTools::Eraser;
                        data.mode = tool;
                        ctx.submit_notification(BUTTON_HIGHLIGHT_COMMAND.with(tool.to_string()));
                        ctx.set_handled();
                    }),
                )
                .cross_axis_alignment(CrossAxisAlignment::End)
                .main_axis_alignment(MainAxisAlignment::Start),
        );

        let right_buttons = WidgetPod::new(
            Flex::row()
                .with_child(
                    ImageButton::new(open_icon, Size::new(26.0, 26.0), String::new()).on_click(
                        move |ctx, _: &mut ApplicationState, _env| {
                            open_from_file(ctx);
                            ctx.set_handled();
                        },
                    ),
                )
                .with_spacer(4.0)
                .with_child(
                    ImageButton::new(save_icon, Size::new(26.0, 26.0), String::new()).on_click(
                        move |ctx, data: &mut ApplicationState, _env| {
                            save_to_file(data, ctx);
                            ctx.set_handled();
                        },
                    ),
                )
                .cross_axis_alignment(CrossAxisAlignment::End)
                .main_axis_alignment(MainAxisAlignment::Start),
        );

        ToolBarWidget {
            left_buttons,
            right_buttons,
        }
    }
}

fn open_from_file(ctx: &mut druid::EventCtx) {
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![FileSpec::TEXT])
        .default_type(FileSpec::TEXT)
        .default_name("diagram.txt")
        .name_label("Source")
        .title("Open diagram")
        .button_text("Open");

    ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options));
}

fn save_to_file(data: &mut ApplicationState, ctx: &mut druid::EventCtx) {
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![FileSpec::TEXT])
        .default_type(FileSpec::TEXT)
        .default_name("diagram")
        .name_label("Destination")
        .title("Save diagram")
        .button_text("Save");

    if let Some(current_file) = &data.current_file {
        ctx.submit_command(druid::commands::SAVE_FILE_AS.with(FileInfo {
            path: PathBuf::from(current_file),
            format: None,
        }));
    } else {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options));
    }
}

impl Widget<ApplicationState> for ToolBarWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut ApplicationState,
        env: &druid::Env,
    ) {
        self.left_buttons.event(ctx, event, data, env);
        self.right_buttons.event(ctx, event, data, env);

        // Prevent the mouse event to be propagated to underlying widgets
        match event {
            Event::WindowConnected => {
                ctx.submit_command(BUTTON_HIGHLIGHT_COMMAND.with(data.mode.to_string()));
            }
            Event::Notification(notification) => {
                if let Some(name) = notification.get(BUTTON_HIGHLIGHT_COMMAND) {
                    ctx.submit_command(BUTTON_HIGHLIGHT_COMMAND.with(name.to_string()));
                }
            }
            Event::KeyDown(event) => {
                if data.mode != DrawingTools::Text && event.mods.meta() || event.mods.ctrl() {
                    match event.code {
                        druid::Code::KeyS => {
                            save_to_file(data, ctx);
                        }
                        druid::Code::KeyO => {
                            open_from_file(ctx);
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &ApplicationState,
        env: &druid::Env,
    ) {
        self.left_buttons.lifecycle(ctx, event, data, env);
        self.right_buttons.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &ApplicationState,
        data: &ApplicationState,
        env: &druid::Env,
    ) {
        if old_data.mode != data.mode {
            ctx.submit_command(BUTTON_HIGHLIGHT_COMMAND.with(data.mode.to_string()));
        }
        self.left_buttons.update(ctx, data, env);
        self.right_buttons.update(ctx, data, env);
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &ApplicationState,
        env: &druid::Env,
    ) -> Size {
        let window_size = ctx.window().get_size();

        self.left_buttons
            .set_origin(ctx, Point::new(26.0, -26.0 + 4.0));

        self.right_buttons.set_origin(
            ctx,
            Point::new(window_size.width - 26.0 * 3.0 - 8.0, -26.0 + 4.0),
        );

        self.left_buttons.layout(ctx, bc, data, env);
        self.right_buttons.layout(ctx, bc, data, env);

        Size {
            width: window_size.width,
            height: 26.0,
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &ApplicationState, env: &druid::Env) {
        let Self {
            left_buttons,
            right_buttons,
        } = &self;

        for area in &[left_buttons, right_buttons] {
            let position = area.layout_rect().origin();
            let content_width = area.layout_rect().width(); //.min(ctx.window().get_size().width - 2.0 * 26.0);
            let size = ctx.size();
            let brush = ctx.solid_brush(Color::from_hex_str("#333333").unwrap());
            let stroke_brush = ctx.solid_brush(Color::from_hex_str("#4c4c4c").unwrap());
            let shadow_brush = ctx.solid_brush(Color::rgba(0.0, 0.0, 0.0, 0.55));
            let rect = Rect::new(
                position.x - 4.0,
                size.height - 53.0,
                position.x - 4.0 + content_width + 12.0,
                size.height - 17.0,
            );
            let shadow_rect = Rect::new(
                position.x - 4.0,
                size.height - 47.0,
                position.x - 4.0 + content_width + 12.0,
                size.height - 17.0,
            );
            ctx.blurred_rect(shadow_rect, 5.0, &shadow_brush);
            ctx.fill(rect.to_rounded_rect(5.0), &brush);
            ctx.stroke(rect.to_rounded_rect(5.0), &stroke_brush, 1.0);
        }

        self.left_buttons.paint(ctx, data, env);
        self.right_buttons.paint(ctx, data, env);
    }
}
