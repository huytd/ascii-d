use super::grid_cell::GridCell;

pub struct GridList {
    data: Vec<GridCell>,
    pub cell_size: (f64, f64),
    pub grid_size: (usize, usize),
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
}
