use druid::{Data, Widget, WidgetPod};
pub struct StackLayout<T>
where
    T: Data,
{
    children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
}

impl<T: Data> StackLayout<T> {
    pub fn new() -> Self {
        StackLayout { children: vec![] }
    }

    pub fn add_child(&mut self, w: impl Widget<T> + 'static) {
        self.children.push(WidgetPod::new(Box::new(w)));
    }
}

impl<T: Data> Widget<T> for StackLayout<T> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        for child in self
            .children
            .iter_mut()
            .rev()
            .filter_map(|x| Some(x.widget_mut()))
        {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        for child in self
            .children
            .iter_mut()
            .filter_map(|x| Some(x.widget_mut()))
        {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, data: &T, env: &druid::Env) {
        for child in self
            .children
            .iter_mut()
            .filter_map(|x| Some(x.widget_mut()))
        {
            child.update(ctx, _old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        for child in self
            .children
            .iter_mut()
            .filter_map(|x| Some(x.widget_mut()))
        {
            child.layout(ctx, bc, data, env);
        }
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        for child in self
            .children
            .iter_mut()
            .filter_map(|x| Some(x.widget_mut()))
        {
            child.paint(ctx, data, env);
        }
    }
}
