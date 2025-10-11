use anyhow::Result;
use ratatui::{
  Terminal,
  crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll, read},
  prelude::CrosstermBackend,
};
use std::{
  io::Stdout,
  ops::ControlFlow,
  thread::sleep,
  time::{Duration, Instant},
};

use crate::cells::Cells;

mod cells;
mod widgets;

fn main() {
  let mut term = ratatui::init();
  let mut state = State::new();

  for x in -1..=1 {
    for y in -1..=1 {
      if (x, y) != (0, 0) {
        state.cells.insert((x, y));
      }
    }
  }

  state.cells.insert((5, 5));
  state.cells.insert((7, 7));

  let result = state.run(&mut term);
  ratatui::restore();

  if let Err(e) = result {
    eprintln!("Error: {}", e);
  }
}

struct State {
  cells: Cells,
  view: View,
  tick_rate: TickRate,
  frame_time: Duration,
  paused: bool,
}

impl State {
  fn new() -> Self {
    Self {
      cells: Cells::new(),
      view: View::default(),
      tick_rate: TickRate::Normal,
      frame_time: Duration::ZERO,
      paused: true,
    }
  }

  fn run(&mut self, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let frame_rate = Duration::from_secs_f64(1. / 60.);
    let mut accumulator = Duration::ZERO;
    let mut last_frame = Instant::now();

    Ok(loop {
      if self.handle_events()?.is_break() {
        break;
      }

      let tick_rate: Duration = self.tick_rate.into();
      let delta = last_frame.elapsed();
      last_frame = Instant::now();

      if !self.paused {
        accumulator += delta;
        while accumulator >= tick_rate {
          self.update()?;
          accumulator -= tick_rate;
        }
      }

      term.draw(|frame| frame.render_widget(&*self, frame.area()))?;

      let elapsed = last_frame.elapsed();
      if elapsed < frame_rate {
        sleep(frame_rate - elapsed);
      }

      self.frame_time = last_frame.elapsed();
    })
  }

  fn handle_events(&mut self) -> Result<ControlFlow<()>> {
    Ok(ControlFlow::Continue(while poll(Duration::default())? {
      let event = read()?;
      if let Event::Key(KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        ..
      }) = event
      {
        match (code, modifiers) {
          (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(ControlFlow::Break(())),
          (KeyCode::Left, KeyModifiers::SHIFT) => self.view.translate.left(),
          (KeyCode::Right, KeyModifiers::SHIFT) => self.view.translate.right(),
          (KeyCode::Up, KeyModifiers::SHIFT) => self.view.translate.up(),
          (KeyCode::Down, KeyModifiers::SHIFT) => self.view.translate.down(),
          (KeyCode::Char('+'), _) => self.view.zoom.inward(),
          (KeyCode::Char('-'), _) => self.view.zoom.outward(),
          _ => {}
        }
      }
    }))
  }

  fn update(&mut self) -> Result<()> {
    Ok(())
  }
}

#[derive(Default)]
struct View {
  cursor: Cursor,
  zoom: ZoomLevel,
  translate: Translate,
}

#[derive(PartialEq, Default)]
struct Cursor {
  offset_row: f64,
  offset_col: f64,
}

impl Cursor {
  fn new() -> Self {
    Self {
      offset_row: 0.,
      offset_col: 0.,
    }
  }

  fn at(&self, offset_row: f64, offset_col: f64) -> bool {
    Cursor {
      offset_row,
      offset_col,
    } == *self
  }
}

#[derive(Default)]
enum ZoomLevel {
  #[default]
  Z1,
  Z2,
  Z3,
  Z4,
}

impl ZoomLevel {
  fn outward(&mut self) {
    *self = match self {
      Self::Z1 => Self::Z2,
      Self::Z2 => Self::Z3,
      Self::Z3 => Self::Z4,
      Self::Z4 => Self::Z4,
    }
  }

  fn inward(&mut self) {
    *self = match self {
      Self::Z1 => Self::Z1,
      Self::Z2 => Self::Z1,
      Self::Z3 => Self::Z2,
      Self::Z4 => Self::Z3,
    }
  }
}

impl From<&ZoomLevel> for f64 {
  fn from(value: &ZoomLevel) -> Self {
    match value {
      ZoomLevel::Z1 => 1.,
      ZoomLevel::Z2 => 2.25,
      ZoomLevel::Z3 => 4.5,
      ZoomLevel::Z4 => 9.,
    }
  }
}

#[derive(Default)]
struct Translate {
  row: f64,
  col: f64,
}

impl Translate {
  fn left(&mut self) {
    self.col += 1.
  }

  fn right(&mut self) {
    self.col -= 1.
  }

  fn up(&mut self) {
    self.row -= 1.
  }

  fn down(&mut self) {
    self.row += 1.
  }
}

#[derive(Clone, Copy)]
enum TickRate {
  Slow,
  Normal,
  Fast,
}

impl TickRate {
  fn increase(&mut self) {
    *self = match *self {
      Self::Slow => Self::Normal,
      Self::Normal => Self::Fast,
      Self::Fast => Self::Slow,
    }
  }

  fn decrease(&mut self) {
    *self = match *self {
      Self::Slow => Self::Fast,
      Self::Normal => Self::Slow,
      Self::Fast => Self::Normal,
    }
  }
}

impl From<TickRate> for Duration {
  fn from(value: TickRate) -> Self {
    Duration::from_secs_f64(match value {
      TickRate::Slow => 1.,
      TickRate::Normal => 1. / 5.,
      TickRate::Fast => 1. / 10.,
    })
  }
}
