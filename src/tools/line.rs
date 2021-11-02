use super::ToolControl;

pub struct LineTool {
    from: Option<(usize, usize)>
}

impl LineTool {
    pub fn new() -> Self {
        Self {
            from: None
        }
    }
}

impl ToolControl for LineTool {
    fn start(&mut self, event: &druid::MouseEvent, cell_size: (f64, f64), grid: (usize, usize)) {
        let (cell_width, cell_height) = cell_size;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        self.from = Some((mouse_row, mouse_col));
    }

    fn draw(
        &mut self,
        event: &druid::MouseEvent,
        buffer: &mut Vec<char>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        let (cell_width, cell_height) = cell_size;
        let (rows, cols) = grid;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        if let Some((from_row, from_col)) = self.from {
            let d_row = (mouse_row as isize - from_row as isize).abs();
            let d_col = (mouse_col as isize - from_col as isize).abs();
            if d_row > d_col {
                // Draw vertical line
                let from = if from_row > mouse_row { mouse_row } else { from_row };
                let to = if from_row > mouse_row { from_row } else { mouse_row };
                for row in from..to {
                    let i = row * cols + from_col;
                    buffer[i] = '*';
                }
            } else {
                // Draw horizontal line
                let from = if from_col > mouse_col { mouse_col } else { from_col };
                let to = if from_col > mouse_col { from_col } else { mouse_col };
                for col in from..to {
                    let i = from_row * cols + col;
                    buffer[i] = '*';
                }
            }
        }
    }

    fn end(
        &mut self,
        event: &druid::MouseEvent,
        buffer: &mut Vec<char>,
        cell_size: (f64, f64),
        grid: (usize, usize),
    ) {
        self.from = None;
    }
}
