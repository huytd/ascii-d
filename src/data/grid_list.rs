use super::{
    grid_cell::GridCell,
    history::{Version, HISTORY_MANAGER},
};
use druid::Rect;
use std::fmt::Display;

pub struct GridList {
    data: Vec<GridCell>,
    pub cell_size: (f64, f64),
    pub grid_size: (usize, usize),
    pub current_selection: Option<((usize, usize), (usize, usize))>,
}

impl Display for GridList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rows, cols) = self.grid_size;
        let mut max_cols = 0;
        let mut max_rows = 0;
        for row in 0..rows {
            for col in 0..cols {
                let i = row * cols + col;
                if !self.data[i].read_content().is_whitespace() {
                    max_cols = max_cols.max(col);
                    max_rows = max_rows.max(row);
                }
            }
        }
        for row in 0..=(max_rows + 1) {
            for col in 0..=(max_cols) {
                let i = row * cols + col;
                write!(f, "{}", self.data[i].read_content())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl GridList {
    pub fn default() -> Self {
        GridList {
            data: vec![],
            cell_size: (0.0, 0.0),
            grid_size: (0, 0),
            current_selection: None,
        }
    }

    pub fn new(cell_width: f64, cell_height: f64, rows: usize, cols: usize) -> Self {
        GridList {
            data: vec![GridCell::empty(); rows * cols],
            cell_size: (cell_width, cell_height),
            grid_size: (rows, cols),
            current_selection: None,
        }
    }

    pub fn clear_all(&mut self) {
        let (rows, cols) = self.grid_size;
        self.data = vec![GridCell::empty(); rows * cols];
    }

    pub fn get(&mut self, index: usize) -> &mut GridCell {
        &mut self.data[index]
    }

    pub fn set(&mut self, index: usize, content: char) {
        self.data[index].content = content;
    }

    pub fn get_highlighted_content(&mut self) -> String {
        let mut last_i: Option<usize> = None;
        let cells = self.data.iter_mut().filter(|cell| cell.highlighted);
        let mut result = vec![];
        let mut line = String::new();
        for cell in cells {
            if let Some(last_i) = last_i {
                if last_i.abs_diff(cell.highlight_index) > 2 {
                    // new line
                    result.push(line.to_owned());
                    line.clear();
                }
            }
            last_i = Some(cell.highlight_index);
            line.push(cell.read().0);
        }
        if !line.is_empty() {
            result.push(line);
        }
        result.join("\n")
    }

    pub fn highlight(&mut self, index: usize) {
        self.clear_all_highlight();
        self.data[index].highlight(index);
    }

    pub fn highlight_rect(&mut self, rect: Rect) {
        self.clear_all_highlight();
        let (_, grid_width) = self.grid_size;
        let (cell_width, cell_height) = self.cell_size;
        let mut start_row = (rect.y0 / cell_height).floor() as usize;
        let mut start_col = (rect.x0 / cell_width).floor() as usize;
        if start_row > 0 {
            start_row += 1;
        }
        if start_col > 0 {
            start_col += 1;
        }
        let end_row = (rect.y1 / cell_height).floor() as usize;
        let end_col = (rect.x1 / cell_width).floor() as usize;

        let sel_width = end_col.saturating_sub(start_col);
        let sel_height = end_row.saturating_sub(start_row);

        for row in 0..sel_height {
            for col in 0..sel_width {
                let index = (start_row + row) * grid_width + (start_col + col);
                self.data[index].highlight(index);
            }
        }

        self.current_selection = Some(((start_row, start_col), (end_row, end_col)));
    }

    pub fn erase_highlighted(&mut self) {
        let mut version = Version::new();
        self.data
            .iter_mut()
            .enumerate()
            .filter(|(_, cell)| cell.highlighted)
            .for_each(|(i, cell)| {
                version.push(i, cell.content, ' ');
                cell.clear();
            });
        unsafe {
            HISTORY_MANAGER.save_version(version);
        }
    }

    pub fn clear_all_highlight(&mut self) {
        for cell in self.data.iter_mut() {
            if cell.highlighted {
                cell.clear_highlight();
            }
        }
        self.current_selection = None;
    }

    pub fn commit_all(&mut self) {
        let mut version = Version::new();
        for (i, cell) in self.data.iter_mut().enumerate() {
            if cell.preview.is_some() {
                let from = cell.content;
                cell.commit();
                version.push(i, from, cell.content);
            }
        }
        unsafe {
            HISTORY_MANAGER.save_version(version);
        }
    }

    pub fn discard_all(&mut self) {
        for cell in self.data.iter_mut() {
            if cell.preview.is_some() {
                cell.discard();
            }
        }
    }

    pub fn put_preview_at(&mut self, content: &str, row: usize, col: usize) {
        let (_, cols) = self.grid_size;
        let mut row = row;
        for line in content.lines() {
            let mut col = col;
            for c in line.chars() {
                if !c.is_whitespace() {
                    let i = row * cols + col;
                    self.data[i].set_preview(c);
                }
                col += 1;
            }
            row += 1;
        }
    }

    pub fn load_content_at(&mut self, content: String, row: usize, col: usize) {
        let mut version = Version::new();
        let (_, cols) = self.grid_size;
        let mut row = row;
        for line in content.lines() {
            let mut col = col;
            for c in line.chars() {
                if !c.is_whitespace() {
                    let i = row * cols + col;
                    version.push(i, self.data[i].content, c);
                    self.data[i].set_content(c);
                }
                col += 1;
            }
            row += 1;
        }
        unsafe {
            HISTORY_MANAGER.save_version(version);
        }
    }

    pub fn load_content(&mut self, content: String) {
        self.load_content_at(content, 0, 0);
    }
}
