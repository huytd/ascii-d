use std::marker::PhantomData;

use druid::{
    theme,
    widget::{Click, ControllerHost, Image},
    Data, Env, Event, EventCtx, ImageBuf, LifeCycle, RenderContext, Size, Widget,
};

use crate::consts::BUTTON_HIGHLIGHT_COMMAND;

pub struct ImageButton<T> {
    image: Image,
    size: Size,
    tag: String,
    highlighted: bool,
    _data: PhantomData<T>,
}

impl<T: Data> ImageButton<T> {
    pub fn new(image_buf: ImageBuf, size: Size, tag: String) -> Self {
        Self {
            image: Image::new(image_buf),
            _data: PhantomData,
            highlighted: false,
            tag,
            size,
        }
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: Data> Widget<T> for ImageButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            Event::Command(cmd) => {
                if let Some(tag) = cmd.get(BUTTON_HIGHLIGHT_COMMAND) {
                    if self.tag.eq(tag) {
                        self.highlighted = true;
                    } else {
                        self.highlighted = false;
                    }
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        _data: &T,
        _env: &Env,
    ) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> druid::Size {
        self.size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
        let is_active = self.highlighted;
        let is_hot = ctx.is_hot();
        let size = ctx.size();
        let stroke_width = env.get(theme::BUTTON_BORDER_WIDTH);

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let border_color = if is_hot && !ctx.is_disabled() {
            env.get(theme::BORDER_LIGHT)
        } else if is_active {
            env.get(theme::PRIMARY_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        self.image.paint(ctx, data, env);
    }
}
