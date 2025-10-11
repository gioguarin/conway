use std::ops::{Deref, DerefMut};

use dashmap::DashSet;

pub struct Cells(DashSet<(i64, i64)>);

impl Cells {
  pub fn new() -> Self {
    Self(DashSet::new())
  }
}

impl Cells {
  pub fn subset(&self, min: (i64, i64), max: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    (min.1..=max.1)
      .flat_map(move |y| (min.0..=max.0).filter_map(move |x| self.get(&(x, y)).map(|r| *r.key())))
  }
}

impl Deref for Cells {
  type Target = DashSet<(i64, i64)>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Cells {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
