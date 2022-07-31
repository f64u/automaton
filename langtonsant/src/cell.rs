use cellular_automaton::cell::BasicCell;
use rand::Rng;

const PROPORTION: f64 = 0.9;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn cw(&self) -> Self {
        match *self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    pub fn ccw(&self) -> Self {
        match *self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CellType {
    CW,
    CCW,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub cell_type: CellType,
    pub value: usize,
}

impl Color {
    pub fn next(&self) -> Self {
        Self {
            cell_type: self.cell_type,
            value: self.value + 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Color(Color),
    Ant(Direction, Color),
}

impl Cell {
    pub fn to_ant(&self) -> Self {
        match *self {
            s @ Self::Ant(_, _) => s,
            Self::Color(c) => Self::Ant(Direction::Left, c),
        }
    }

    pub fn to_color(&self) -> Self {
        match *self {
            s @ Self::Color(_) => s,
            Self::Ant(_, c) => Self::Color(c),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::Color(Color {
            value: 0,
            cell_type: CellType::CW,
        })
    }
}

impl BasicCell for Cell {
    fn next_state(&self) -> Self {
        match *self {
            Self::Ant(d, c) => {
                if matches!(d, Direction::Down) {
                    self.to_color()
                } else {
                    Self::Ant(d.cw(), c)
                }
            }
            Self::Color(c) => Self::Color(c.next()),
        }
    }

    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        if rng.gen_bool(PROPORTION) {
            Cell::Color(Color {
                cell_type: if rng.gen_bool(0.5) {
                    CellType::CW
                } else {
                    CellType::CCW
                },
                value: 0,
            })
        } else {
            Cell::Color(Color {
                cell_type: if rng.gen_bool(0.5) {
                    CellType::CW
                } else {
                    CellType::CCW
                },
                value: 1,
            })
        }
    }
}

impl Cell {
    pub fn random_with_pattern<R: Rng + ?Sized>(rng: &mut R, pattern: &Vec<CellType>) -> Self {
        let v = rng.gen_range(0..pattern.len());
        Self::Color(Color {
            value: v,
            cell_type: pattern[v],
        })
    }
}
