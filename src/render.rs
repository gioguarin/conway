use ratatui::{
  buffer::Buffer,
  layout::{Alignment, Rect},
  style::{Color, Stylize},
  symbols::Marker,
  widgets::{
    Block, Paragraph, Widget,
    canvas::{Canvas, Points},
  },
};

use crate::State;

const CONTROLS: &str = " - pan: shift+arrows | move: arrows | zoom: z | place: space | pause: p | cursor: c | help: h - ";

impl Widget for &mut State {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let c = &mut self.view.cursor;

    let block = Block::new()
      .title_top(CONTROLS)
      .title_bottom(format!(
        "- cycle cursor pattern: [, ] - current: {}",
        c.pattern.name()
      ))
      .title_alignment(Alignment::Center)
      .bg(Color::DarkGray);

    let (w, h) = if self.view.controls {
      (
        block.inner(area).width as f64,
        block.inner(area).height as f64,
      )
    } else {
      (area.width as f64, area.height as f64)
    };

    self.view.bounds = if self.view.zoom {
      (w / 2., h)
    } else {
      (w, h * 2.)
    };

    let t_col = self.view.translate.col;
    let t_row = self.view.translate.row;
    let (x_range, y_range) = self.view.bounds;
    c.offset_col = c.offset_col.clamp(-x_range, x_range);
    c.offset_row = c.offset_row.clamp(-y_range, y_range);
    let canvas = Canvas::default()
      .marker(if self.view.zoom {
        Marker::HalfBlock
      } else {
        Marker::Braille
      })
      .x_bounds([t_col - x_range, t_col + x_range])
      .y_bounds([t_row - y_range, t_row + y_range])
      .paint(|ctx| {
        ctx.draw(&Points {
          coords: &self
            .cells
            .subset(
              (
                (t_col - x_range).floor() as i64,
                (t_row - y_range).floor() as i64,
              ),
              (
                (t_col + x_range).ceil() as i64,
                (t_row + y_range).ceil() as i64,
              ),
            )
            .map(|(x, y)| (x as f64, y as f64))
            .collect::<Vec<_>>(),
          color: Color::White,
        });
        if !c.hidden && self.view.zoom {
          ctx.draw(&Points {
            coords: &c
              .pattern
              .coords()
              .into_iter()
              .map(|(col, row)| {
                (
                  c.offset_col + t_col + col as f64,
                  c.offset_row + t_row + row as f64,
                )
              })
              .collect::<Vec<_>>(),
            color: Color::Cyan,
          })
        }
      });
    if self.view.controls {
      canvas.block(block).render(area, buf)
    } else {
      canvas.render(area, buf)
    }
  }
}
