pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Default)]
pub struct View {
  pub controls_hidden: bool,
  pub bounds: (f64, f64),
  pub cursor: Cursor,
  pub translate: Translate,
}

impl View {
  pub fn move_cursor(&mut self, dir: Direction) {
    let (c_col, c_row) = (&mut self.cursor.offset_col, &mut self.cursor.offset_row);
    let (col_range, row_range) = (self.bounds.0, self.bounds.1);
    match dir {
      Direction::Left => {
        if *c_col <= -col_range + 1. {
          *c_col = col_range;
        } else {
          *c_col -= 1.;
        }
      }
      Direction::Right => {
        if *c_col >= col_range {
          *c_col = -col_range + 1.;
        } else {
          *c_col += 1.;
        }
      }
      Direction::Up => {
        if *c_row >= row_range - 1. {
          *c_row = -row_range;
        } else {
          *c_row += 1.;
        }
      }
      Direction::Down => {
        if *c_row <= -row_range {
          *c_row = row_range - 1.
        } else {
          *c_row -= 1.;
        }
      }
    }
  }
}

#[derive(PartialEq, Default)]
pub struct Cursor {
  pub hidden: bool,
  pub offset_row: f64,
  pub offset_col: f64,
}

impl Cursor {
  pub fn at(&self, offset_row: f64, offset_col: f64) -> bool {
    (self.offset_row, self.offset_col) == (offset_row, offset_col)
  }

  pub fn toggle(&mut self) {
    self.hidden = !self.hidden
  }
}

#[derive(Default)]
pub struct Translate {
  pub row: f64,
  pub col: f64,
}

impl Translate {
  pub fn left(&mut self) {
    self.col -= 1.
  }

  pub fn right(&mut self) {
    self.col += 1.
  }

  pub fn up(&mut self) {
    self.row += 1.
  }

  pub fn down(&mut self) {
    self.row -= 1.
  }
}
