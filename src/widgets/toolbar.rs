use super::image_button::ImageButton;
use crate::{consts::BUTTON_HIGHLIGHT_COMMAND, data::ApplicationState, tools::DrawingTools};
use druid::{
    widget::{CrossAxisAlignment, Flex, MainAxisAlignment},
    Color, Event, ImageBuf, Point, Rect, RenderContext, Size, Widget, WidgetPod,
};

pub struct ToolBarWidget {
    buttons: WidgetPod<ApplicationState, Flex<ApplicationState>>,
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
        let pod = WidgetPod::new(
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
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .main_axis_alignment(MainAxisAlignment::Start),
        );
        ToolBarWidget { buttons: pod }
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
        self.buttons.event(ctx, event, data, env);
        // Prevent the mouse event to be propagated to underlying widgets
        match event {
            Event::WindowConnected => {
                ctx.submit_command(BUTTON_HIGHLIGHT_COMMAND.with(data.mode.to_string()));
            }
            Event::MouseDown(event) | Event::MouseUp(event) | Event::MouseMove(event) => {
                let size = ctx.size();
                let content_width = self.buttons.layout_rect().width();
                let rect = Rect::new(
                    20.0,
                    size.height - 53.0,
                    20.0 + content_width + 12.0,
                    size.height - 17.0,
                );
                if rect.contains(event.pos) {
                    ctx.set_handled();
                }
            }
            Event::Notification(notification) => {
                if let Some(name) = notification.get(BUTTON_HIGHLIGHT_COMMAND) {
                    ctx.submit_command(BUTTON_HIGHLIGHT_COMMAND.with(name.to_string()));
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
        self.buttons.lifecycle(ctx, event, data, env);
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
        self.buttons.update(ctx, data, env);
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &ApplicationState,
        env: &druid::Env,
    ) -> Size {
        let size = ctx.window().get_size();
        let margin = Size::new(26.0, 53.0 - 4.0);
        self.buttons.layout(ctx, bc, data, env);
        self.buttons
            .set_origin(ctx, data, env, Point::new(margin.width, size.height - margin.height));
        Size {
            width: 100.0,
            height: 26.0,
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &ApplicationState, env: &druid::Env) {
        let content_width = self.buttons.layout_rect().width();
        let size = ctx.size();
        let brush = ctx.solid_brush(Color::from_hex_str("#333333").unwrap());
        let stroke_brush = ctx.solid_brush(Color::from_hex_str("#4c4c4c").unwrap());
        let shadow_brush = ctx.solid_brush(Color::rgba(0.0, 0.0, 0.0, 0.55));
        let rect = Rect::new(
            20.0,
            size.height - 53.0,
            20.0 + content_width + 12.0,
            size.height - 17.0,
        );
        let shadow_rect = Rect::new(
            20.0,
            size.height - 47.0,
            20.0 + content_width + 12.0,
            size.height - 17.0,
        );
        ctx.blurred_rect(shadow_rect, 5.0, &shadow_brush);
        ctx.fill(rect.to_rounded_rect(5.0), &brush);
        ctx.stroke(rect.to_rounded_rect(5.0), &stroke_brush, 1.0);

        self.buttons.paint(ctx, data, env);
    }
}
