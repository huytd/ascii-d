macro_rules! impl_shape_for {
    ($($t:ty),+ $(,)?) => ($(
        use crate::shapes::Shape;
        impl Shape for $t {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn get_points(&self) -> (Point, Point) {
                (
                    Point::from((self.start.0 as f64, self.start.1 as f64)),
                    Point::from((self.end.0 as f64, self.end.1 as f64)),
                )
            }
        }
    )+)
}
