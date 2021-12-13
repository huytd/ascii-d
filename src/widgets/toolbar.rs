use std::ops::Add;

use druid::{
    piet::StrokeStyle,
    widget::{Button, CrossAxisAlignment, Flex, MainAxisAlignment},
    BoxConstraints, Color, Event, FontDescriptor, FontFamily, MouseEvent, Point, Rect,
    RenderContext, Size, TextLayout, Widget, WidgetPod,
};

use crate::data::ApplicationState;

pub struct ToolBarWidget {
    buttons: WidgetPod<ApplicationState, Flex<ApplicationState>>,
}

impl ToolBarWidget {
    pub fn new() -> Self {
        let pod = WidgetPod::new(
            Flex::row()
                .with_child(Button::new("Select").on_click(|ctx, data, env| {
                    println!("You clicked");
                    ctx.set_handled();
                }))
                .with_spacer(4.0)
                .with_child(Button::new("Line"))
                .with_spacer(4.0)
                .with_child(Button::new("Text"))
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
        self.buttons.layout(ctx, bc, data, env);
        self.buttons
            .set_origin(ctx, data, env, Point::new(26.0, size.height - 75.0));
        Size {
            width: 100.0,
            height: 20.0,
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
