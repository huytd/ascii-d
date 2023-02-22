use std::fmt::Display;

use super::grid_cell::GridCell;

pub struct GridList {
    data: Vec<GridCell>,
    pub cell_size: (f64, f64),
    pub grid_size: (usize, usize),
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
        }
    }

    pub fn new(cell_width: f64, cell_height: f64, rows: usize, cols: usize) -> Self {
        GridList {
            data: vec![GridCell::empty(); rows * cols],
            cell_size: (cell_width, cell_height),
            grid_size: (rows, cols),
        }
    }

    pub fn clear_all(&mut self) {
        let (rows, cols) = self.grid_size;
        self.data = vec![GridCell::empty(); rows * cols];
    }

    pub fn get(&mut self, index: usize) -> &mut GridCell {
        &mut self.data[index]
    }

    pub fn highlight(&mut self, index: usize) {
        self.clear_all_highlight();
        self.data[index].highlight();
    }

    pub fn clear_all_highlight(&mut self) {
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

    pub fn load_content(&mut self, content: String) {
        let (_, cols) = self.grid_size;
        let mut row = 0;
        for line in content.lines() {
            let mut col = 0;
            for c in line.chars() {
                if !c.is_whitespace() {
                    let i = row * cols + col;
                    self.data[i].set_content(c);
                }
                col += 1;
            }
            row += 1;
        }
    }
}
