use druid::Data;

use crate::consts::{CHAR_NEWLINE, CHAR_SPACE};

#[derive(Clone, PartialEq, Data)]
pub struct ApplicationState {
    pub mode: String,
}

#[derive(Clone, Copy)]
pub struct GridCell {
    pub content: char,
    pub preview: Option<char>,
    pub highlighted: bool,
}

impl GridCell {
    pub fn new(content: char) -> Self {
        Self {
            content,
            preview: None,
            highlighted: false,
        }
    }

    pub fn empty() -> Self {
        GridCell::new(CHAR_SPACE)
    }

    pub fn newline() -> Self {
        GridCell::new(CHAR_NEWLINE)
    }

    pub fn read(&self) -> char {
        if let Some(content) = self.preview {
            return content;
        }
        self.content
    }

    pub fn read_content(&self) -> char {
        self.content
    }

    pub fn clear(&mut self) {
        self.content = CHAR_SPACE;
        self.preview = None;
    }

    pub fn set_content(&mut self, content: char) {
        self.content = content;
    }

    pub fn set_preview(&mut self, content: char) {
        self.preview = Some(content);
    }

    pub fn commit(&mut self) {
        if let Some(preview) = self.preview {
            self.content = preview;
            self.preview = None;
        }
    }

    pub fn discard(&mut self) {
        self.preview = None;
    }

    pub fn highlight(&mut self) {
        self.highlighted = true;
    }

    pub fn clear_highlight(&mut self) {
        self.highlighted = false;
    }
}

pub struct GridList {
    data: Vec<GridCell>,
}

impl GridList {
    pub fn new(cap: usize) -> Self {
        GridList {
            data: vec![GridCell::empty(); cap],
        }
    }

    pub fn get(&mut self, index: usize) -> &mut GridCell {
        &mut self.data[index]
    }

    pub fn highlight(&mut self, index: usize) {
        self.clear_highlight_all();
        self.data[index].highlight();
    }

    pub fn clear_highlight_all(&mut self) {
        for cell in self.data.iter_mut() {
            if cell.highlighted {
                cell.clear_highlight();
            }
        }
    }

    pub fn commit_all(&mut self) {
        for cell in self.data.iter_mut() {
            if cell.preview.is_some() {
                cell.commit();
            }
        }
    }

    pub fn discard_all(&mut self) {
        for cell in self.data.iter_mut() {
            if cell.preview.is_some() {
                cell.discard();
            }
        }
    }
}
