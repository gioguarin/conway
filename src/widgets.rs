use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::Color,
  widgets::{
    Widget,
    canvas::{Canvas, Painter, Points, Rectangle, Shape},
  },
};

use crate::State;

impl Widget for &State {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let aspect_ratio = (area.width as f64) / (area.height as f64) / 2.0;
    let y_range = 10. * f64::from(&self.view.zoom);
    let x_range = y_range * aspect_ratio;

    Canvas::default()
      .x_bounds([-x_range, x_range])
      .y_bounds([-y_range, y_range])
      .paint(|ctx| {
        let min_x = (-x_range - self.view.translate.col).floor() as i64 - 10;
        let max_x = (x_range - self.view.translate.col).ceil() as i64 + 10;
        let min_y = (-y_range - self.view.translate.row).floor() as i64 - 10;
        let max_y = (y_range - self.view.translate.row).ceil() as i64 + 10;

        for (x, y) in self.cells.subset((min_x, min_y), (max_x, max_y)) {
          ctx.draw(&FilledRectangle {
            x: x as f64 + self.view.translate.col,
            y: y as f64 + self.view.translate.row,
            width: 1.,
            height: 1.,
            color: Color::White,
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
}

impl Shape for FilledRectangle {
  fn draw(&self, painter: &mut Painter) {
    if let Some((x1, y1)) = painter.get_point(self.x, self.y) {
      if let Some((x2, y2)) = painter.get_point(self.x + self.width, self.y + self.height) {
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);

        for x in x_min..x_max {
          for y in y_min..y_max {
            painter.paint(x, y, self.color);
          }
        }
      }
    }
  }
}
