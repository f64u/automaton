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
pub enum CellVariant {
    Color(Color),
    Ant(Direction, Color),
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub pattern: &'static [CellType],
    pub variant: CellVariant,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            pattern: &[CellType::CW, CellType::CCW],
            variant: CellVariant::Color(Color {
                value: 0,
                cell_type: CellType::CW,
            }),
        }
    }
}

impl BasicCell for Cell {
    fn next_state(&self) -> Self {
        match *self {
            Cell {
                pattern,
                variant: CellVariant::Color(c),
            } => {
                let n = pattern.len();
                let new_c = c.next();
                if new_c.value == n {
                    Cell {
                        pattern,
                        variant: CellVariant::Ant(
                            Direction::Right,
                            Color {
                                value: 0,
                                cell_type: pattern[0],
                            },
                        ),
                    }
                } else {
                    Cell {
                        pattern,
                        variant: CellVariant::Color(new_c),
                    }
                }
            }
            Cell {
                pattern,
                variant: CellVariant::Ant(d, c),
            } => {
                let n = self.pattern.len();
                if c.value < n - 1 {
                    Cell {
                        pattern,
                        variant: CellVariant::Ant(d, c.next()),
                    }
                } else if matches!(d, Direction::Left) {
                    Cell {
                        pattern,
                        variant: CellVariant::Color(c),
                    }
                } else {
                    let new_d = match d {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => unreachable!(),
                        Direction::Up => Direction::Right,
                    };
                    Cell {
                        pattern,
                        variant: CellVariant::Ant(new_d, c),
                    }
                }
            }
        }
    }

    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let f: f64 = rng.gen();

        let t: CellType;
        let v: usize;
        if f < PROPORTION {
            t = CellType::CW;
            v = 0;
        } else {
            t = CellType::CCW;
            v = 1;
        }

        Cell {
            pattern: &[CellType::CW, CellType::CCW],
            variant: CellVariant::Color(Color {
                value: v,
                cell_type: t,
            }),
        }
    }
}
