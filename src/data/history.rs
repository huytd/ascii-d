use super::grid_list::GridList;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
struct Edit {
    index: usize,
    from: char,
    to: char,
}

impl Edit {
    pub fn new(index: usize, from: char, to: char) -> Self {
        Self { index, from, to }
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    edits: Vec<Edit>,
}

impl Version {
    pub fn new() -> Self {
        Self { edits: vec![] }
    }

    pub fn push(&mut self, index: usize, from: char, to: char) {
        self.edits.push(Edit::new(index, from, to));
    }

    pub fn clear(&mut self) {
        self.edits.clear();
    }

    pub fn len(&self) -> usize {
        self.edits.len()
    }
}

pub struct History {
    versions: Vec<Version>,
    // redo_stack: Vec<Version>,
    index: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            versions: vec![],
            // redo_stack: vec![],
            index: 0,
        }
    }

    pub fn save_version(&mut self, version: Version) {
        if version.len() > 0 {
            if self.index + 1 >= self.versions.len() {
                // Push new history
                self.versions.push(version);
            } else {
                // Overwriting history
                self.versions = self.versions[0..self.index].to_vec();
                self.versions.push(version);
            }
            self.index = self.versions.len();
        }
        // println!("{:?}\n", version);
        // self.versions.push(version);
        // if !self.redo_stack.is_empty() {
        //     self.redo_stack.clear();
        // }
    }

    pub fn undo(&mut self, grid_list: &mut GridList) {
        if self.index > 0 {
            self.index -= 1;
            let version = &self.versions[self.index];
            for edit in &version.edits {
                grid_list.set(edit.index, edit.from);
            }
        }
        // if let Some(top) = self.versions.pop() {
        //     println!("{:?}\n", top);
        //     for edit in &top.edits {
        //         grid_list.set(edit.index, edit.from);
        //     }
        //     self.redo_stack.push(top);
        // }
    }

    pub fn redo(&mut self, grid_list: &mut GridList) {
        if self.index < self.versions.len() {
            let version = &self.versions[self.index];
            for edit in &version.edits {
                grid_list.set(edit.index, edit.to);
            }
            self.index += 1;
        }
        // if let Some(top) = self.redo_stack.pop() {
        //     println!("{:?}\n", top);
        //     for edit in &top.edits {
        //         grid_list.set(edit.index, edit.to);
        //     }
        //     self.versions.push(top);
        // }
    }
}

pub static mut HISTORY_MANAGER: Lazy<History> = Lazy::new(History::new);
