use ahash::{HashSet, HashSetExt};
use std::ops::{Deref, DerefMut};

pub struct Cells(HashSet<(i64, i64)>);

impl Cells {
  pub fn new() -> Self {
    Self(HashSet::new())
  }
}

impl Cells {
  pub fn subset(&self, min: (i64, i64), max: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    (min.1..=max.1)
      .flat_map(move |y| (min.0..=max.0).filter_map(move |x| self.get(&(x, y)).copied()))
  }

  pub fn count_neighbors(&self, x: i64, y: i64) -> usize {
    (-1..=1)
      .flat_map(|dx| {
        (-1..=1).filter_map(move |dy| {
          if !(dx == 0 && dy == 0) && self.contains(&(x + dx, y + dy)) {
            Some(())
          } else {
            None
          }
        })
      })
      .count()
  }
}

impl Deref for Cells {
  type Target = HashSet<(i64, i64)>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Cells {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
