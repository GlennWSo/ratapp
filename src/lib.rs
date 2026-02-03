mod soduko;

use crossterm::event::KeyModifiers;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table, TableState,
    },
};
use style::palette::tailwind;

use crate::soduko::BoardState;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];
const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right",
    "(Shift + →) next color | (Shift + ←) previous color",
];

const ITEM_HEIGHT: usize = 4;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

type SodukoData = BoardState;
pub struct App {
    state: TableState,
    data: SodukoData,
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

pub type Result = color_eyre::Result<()>;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(9 + 2),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            data: SodukoData::default(),
        }
    }
    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= 9 - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    9 - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                    KeyCode::Char('l') | KeyCode::Right if shift_pressed => self.next_color(),
                    KeyCode::Char('h') | KeyCode::Left if shift_pressed => {
                        self.previous_color();
                    }
                    KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                    KeyCode::Backspace | KeyCode::Delete => {
                        let Some((r, col)) = self.state.selected_cell() else {
                            continue;
                        };
                        self.data.set(r as u8, col as u8, 0.into());
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let Some((r, col)) = self.state.selected_cell() else {
                            continue;
                        };
                        let d = c.to_digit(10).unwrap() as u8;
                        self.data.set(r as u8, col as u8, d.into());
                        // self.data[r][col] = c.to_digit(10).map(|d| d as u8).unwrap().into();
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(9 + 2 + 2),
            Constraint::Fill(1),
        ]);
        let vertical_areas = vertical.split(frame.area());
        let grid_row = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(4 * 9),
            Constraint::Fill(1),
        ])
        .vertical_margin(1)
        .split(vertical_areas[1]);

        self.set_colors();

        self.render_header(frame, vertical_areas[0]);
        self.render_table(frame, grid_row[1]);
        // self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, vertical_areas[2]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let rows = self.data.iter().enumerate().map(|(r, data)| {
            let color = match r % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let base_style = Style::new().fg(self.colors.row_fg).bg(color);
            let underline = (r + 1) % 3 == 0 && (r + 1) < 9;
            let style = if underline {
                // base_style.add_modifier(Modifier::UNDERLINED)
                base_style
            } else {
                base_style
            };
            data.iter()
                .enumerate()
                .map(|(col, content)| {
                    let mut text = Text::from(format!("{content}"));
                    if (col + 1) % 3 == 0 && (col + 1) < 9 {
                        text.push_span(" |");
                        text = text.right_aligned();
                    } else {
                        text = text.centered();
                    }
                    if underline {
                        text.push_line("----");
                    }
                    Cell::from(text)
                })
                .collect::<Row>()
                .style(style)
                .height(if underline { 2 } else { 1 })
        });
        let t = Table::new(
            rows,
            [
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
                //
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
                //
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
            ],
        )
        // .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        // .highlight_symbol(Text::from(vec![
        //     "".into(),
        //     bar.into(),
        //     bar.into(),
        //     "".into(),
        // ]))
        .bg(self.colors.buffer_bg)
        .column_spacing(0);
        // .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }

    #[allow(dead_code)]
    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let lay = Layout::vertical([
            Constraint::Fill(3),
            Constraint::Length(1),
            Constraint::Max(2),
        ])
        .split(area);
        frame.render_widget(
            Paragraph::new("Soduku")
                .style(header_style.add_modifier(Modifier::BOLD))
                .centered(),
            lay[1],
        );
    }
    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
}

#[cfg(test)]
mod tests {}
