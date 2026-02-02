use std::{default, io, num::NonZeroU8};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, Paragraph, Table, TableState, Widget},
};

type Row = [Option<NonZeroU8>; 9];
type Mat9x9 = [Row; 9];

#[derive(Debug, Default)]
pub struct App {
    grid: Mat9x9,
    exit: bool,
    grid_state: TableState,
}

impl App {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.handle_events()?
        }
        Ok(())
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let rows = (1..=9);
    }

    fn draw(&mut self, frame: &mut Frame) {
        // let area = frame.area();

        // let table = Table::new(rows, widths);

        // for (i, cell) in cells.enumerate() {
        //     Paragraph::new(format!("{i}")).render(cell, buf);
        // }
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement(),
            KeyCode::Right => self.increment(),
            _ => {}
        }
    }
    fn handle_mouse_event(&mut self, event: MouseEvent) {
        match event.kind {
            event::MouseEventKind::Down(mouse_button) => {}
            // event::MouseEventKind::Up(mouse_button) => todo!(),
            // event::MouseEventKind::Drag(mouse_button) => todo!(),
            // event::MouseEventKind::Moved => todo!(),
            // event::MouseEventKind::ScrollDown => todo!(),
            // event::MouseEventKind::ScrollUp => todo!(),
            // event::MouseEventKind::ScrollLeft => todo!(),
            // event::MouseEventKind::ScrollRight => todo!(),
            _ => {}
        }
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            Event::Mouse(m) => self.handle_mouse_event(m),
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement(&mut self) {
        // self.counter = self.counter.saturating_sub(1);
    }

    fn increment(&mut self) {
        // self.counter = self.counter.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        // assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        // assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}
