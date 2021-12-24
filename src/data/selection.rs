use druid::{Point, Rect};

pub struct SelectionRange {
    start: Option<Point>,
    end: Option<Point>,
}

impl SelectionRange {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn set_start(&mut self, p: Point) {
        self.start = Some(p);
    }

    pub fn set_end(&mut self, p: Point) {
        self.end = Some(p);
    }

    pub fn discard(&mut self) {
        self.start = None;
        self.end = None;
    }

    pub fn as_rect(&self) -> Option<Rect> {
        if self.start.is_some() && self.end.is_some() {
            return Some(Rect::from_points(self.start.unwrap(), self.end.unwrap()));
        }
        None
    }
}
