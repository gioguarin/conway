use std::collections::HashSet;

use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::Color,
  widgets::{
    Widget,
    canvas::{Canvas, Painter, Shape},
  },
};

use crate::State;

impl Widget for &State {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let dots_x = area.width as f64 * 2.0;
    let dots_y = area.height as f64 * 4.0;

    let y_range = 10. * f64::from(&self.view.zoom);

    let units_per_dot = (y_range * 2.0) / dots_y;
    let x_range = (dots_x * units_per_dot) / 2.0;

    let (v_col, v_row) = (self.view.translate.col, self.view.translate.row);

    Canvas::default()
      .x_bounds([-x_range, x_range])
      .y_bounds([-y_range, y_range])
      .paint(|ctx| {
        let min_x = (-x_range - v_col).floor() as i64;
        let max_x = (x_range - v_col).ceil() as i64;
        let min_y = (-y_range - v_row).floor() as i64;
        let max_y = (y_range - v_row).ceil() as i64;

        let dpu = 1.0 / units_per_dot;

        for (x, y) in self.cells.subset((min_x, min_y), (max_x, max_y)) {
          ctx.draw(&FilledRectangle {
            x: x as f64 + self.view.translate.col,
            y: y as f64 + self.view.translate.row,
            width: 1.,
            height: 1.,
            color: Color::White,
            dpu,
          });
        }
      })
      .render(area, buf)
  }
}

pub struct FilledRectangle {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  pub color: Color,
  pub dpu: f64,
}

impl Shape for FilledRectangle {
  fn draw(&self, painter: &mut Painter) {
    let left = self.x;
    let bottom = self.y;

    let dots_spanned = self.width * self.dpu;
    let samples = (dots_spanned * 3.0).max(10.0).min(50.0) as usize;

    let mut points = HashSet::new();

    for i in 0..=samples {
      for j in 0..=samples {
        let x = left + (i as f64 / samples as f64) * self.width;
        let y = bottom + (j as f64 / samples as f64) * self.height;

        if let Some(point) = painter.get_point(x, y) {
          points.insert(point);
        }
      }
    }

    for (x, y) in points {
      painter.paint(x, y, self.color);
    }
  }
}
