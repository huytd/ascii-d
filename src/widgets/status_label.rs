use druid::{
    Color, FontDescriptor, FontFamily, Point, Rect, RenderContext, Size, TextLayout, Widget,
};

use crate::data::ApplicationState;

pub struct StatusLabel {
    text: TextLayout<String>,
}

impl StatusLabel {
    pub fn new() -> Self {
        StatusLabel {
            text: TextLayout::from_text("Are you ok?"),
        }
    }

    fn update_label(&mut self, data: &ApplicationState) {
        let text = data.mode.to_string();
        self.text
            .set_font(FontDescriptor::new(FontFamily::MONOSPACE).with_size(16.0));
        self.text.set_text(text);
        self.text.set_text_color(Color::BLACK);
    }
}

impl Widget<ApplicationState> for StatusLabel {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut ApplicationState,
        _env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &ApplicationState,
        _env: &druid::Env,
    ) {
        match event {
            druid::LifeCycle::WidgetAdded => self.update_label(data),
            _ => {}
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &ApplicationState,
        data: &ApplicationState,
        _env: &druid::Env,
    ) {
        self.update_label(data);
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &ApplicationState,
        env: &druid::Env,
    ) -> Size {
        self.text.rebuild_if_needed(ctx.text(), env);
        Size {
            width: 100.0,
            height: 20.0,
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &ApplicationState, env: &druid::Env) {
        self.text.rebuild_if_needed(ctx.text(), env);

        let size = ctx.size();
        let text_size = self.text.layout_metrics();
        let brush = ctx.solid_brush(Color::WHITE);
        let rect = Rect::new(
            20.0,
            size.height - 50.0,
            20.0 + text_size.size.width + 20.0,
            size.height - 20.0,
        )
        .to_rounded_rect(5.0);
        ctx.fill(rect, &brush);

        let text_pos = Point::new(30.0, size.height - text_size.size.height - 25.0);
        self.text.draw(ctx, text_pos);
    }
}
