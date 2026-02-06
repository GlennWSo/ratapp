use std::{
    fmt::{Display, Formatter},
    num::{NonZero, NonZeroU8},
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

#[derive(Default, Debug, Clone, Copy)]
pub struct CellState(Option<NonZeroU8>);
impl Deref for CellState {
    type Target = Option<NonZeroU8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CellState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self(None),
            v @ 1..=9 => Self(NonZeroU8::new(v)),
            10.. => panic!("max value in soduku is 9"),
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(v) => write!(f, "{}", v),
            None => write!(f, "·"),
        }
    }
}

type Soduko9 = [CellState; 9];

#[derive(Default, Debug, Clone, Copy)]
pub struct BoardState([Soduko9; 9]);

impl Deref for BoardState {
    type Target = [Soduko9; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoardState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn unique(data: &[CellState]) -> bool {
    for n in (1..=9).map(|n| NonZeroU8::new(n)) {
        if data.iter().filter(|v| ***v == n).count() > 1 {
            return false;
        }
    }
    true
}

impl BoardState {
    fn square(&self, row: usize, col: usize) -> Soduko9 {
        let rows = row..(row + 3);
        let data: Vec<_> = rows
            .flat_map(|row| {
                let row = self.0[row];
                row.into_iter().skip(col).take(3)
            })
            .collect();
        data.try_into().unwrap()
    }
    fn check_boxes(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                let data = self.square(row * 3, col * 3);
                if !unique(&data) {
                    return false;
                }
            }
        }
        true
    }

    fn column(&self, column: usize) -> Soduko9 {
        (0..9)
            .map(|row| self.0[row][column])
            .collect_array()
            .unwrap()
    }
    fn check_columns(&self) -> bool {
        (0..9).all(|col| unique(&self.column(col)))
    }
    fn check_rows(&self) -> bool {
        (0..9).all(|i| unique(&self.0[i]))
    }
    pub fn check(&self) -> bool {
        self.check_rows() && self.check_columns() && self.check_boxes()
    }
    fn next_cell(&self) -> Option<usize> {
        self.0
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(idx, num)| match num.0 {
                Some(_) => None,
                None => Some(idx),
            })
            .next()
    }
    pub fn set(&mut self, row: u8, col: u8, n: CellState) {
        self.0[row as usize][col as usize] = n;
    }

    pub fn set_pos(&mut self, pos: usize, n: CellState) {
        let row = pos / 9;
        let col = pos % 9;
        self.0[row][col] = n;
    }

    pub fn solve(mut self) -> Option<Self> {
        if !self.check() {
            return None;
        }
        let Some(next_cell) = self.next_cell() else {
            return Some(self);
        };
        for number in (1..=9) {
            self.set_pos(next_cell, number.into());
            if let Some(solution) = self.solve() {
                return Some(solution);
            }
        }
        None
    }
    pub fn solvable(&self) -> bool {
        self.solve().is_some()
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let div = "-------------------------------------";
        for row in self.0 {
            writeln!(f, "{div}")?;
            for num in row {
                let x = match num.0 {
                    Some(v) => format!("{v}"),
                    None => "-".into(),
                };
                write!(f, "| {x} ")?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "{div}")
    }
}
