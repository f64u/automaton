pub mod cell {
    use cellular_automaton::cell::BasicCell;
    use rand::Rng;

    use crate::PROPORTION;

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
}

pub mod world {
    use cellular_automaton::{
        common::{linearize, Dimensions, DoubleVec, Index},
        world::BasicWorld,
    };
    use rand::Rng;

    use super::cell::{Cell, CellType, Color, Direction};

    pub struct World {
        cells: DoubleVec<Cell>,
        dimensions: Dimensions,
        delta: Vec<(Index, Cell)>,
        ant_pos: Index,
        pub pattern: Vec<CellType>,
    }

    impl BasicWorld<Cell> for World {
        fn new(mut cells: DoubleVec<Cell>, dimensions: Dimensions) -> Self {
            let mut ant_pos = None;
            'outer: for (j, row) in cells.iter().enumerate() {
                for (i, c) in row.iter().enumerate() {
                    if matches!(c, Cell::Ant(_, _),) {
                        ant_pos = Some((i, j));
                        break 'outer;
                    }
                }
            }

            if ant_pos.is_none() {
                ant_pos = Some((dimensions.0 / 2, dimensions.1 / 2));
                let ant_pos = ant_pos.unwrap();
                let c = &mut cells[ant_pos.1][ant_pos.0];
                *c = c.to_ant();
            }

            let delta = linearize(cells.clone());

            Self {
                cells,
                dimensions,
                delta,
                ant_pos: ant_pos.unwrap(),
                pattern: vec![CellType::CW, CellType::CCW],
            }
        }

        fn cells(&self) -> &DoubleVec<Cell> {
            &self.cells
        }

        fn cells_mut(&mut self) -> &mut DoubleVec<Cell> {
            &mut self.cells
        }

        fn changes(&self) -> Vec<(Index, Cell)> {
            let mut delta = Vec::with_capacity(2);
            let ant = self.cells()[self.ant_pos.1][self.ant_pos.0];
            if let Cell::Ant(d, c) = ant {
                let value = (c.value + 1) % self.pattern.len();
                delta.push((
                    self.ant_pos,
                    Cell::Color(Color {
                        value,
                        cell_type: self.pattern[value],
                    }),
                ));
                let new_direction = match self.pattern[c.value] {
                    CellType::CW => d.cw(),
                    CellType::CCW => d.ccw(),
                };

                let new_ant_pos = match new_direction {
                    Direction::Right => {
                        ((self.ant_pos.0 + 1) % self.dimensions().0, self.ant_pos.1)
                    }
                    Direction::Down => (self.ant_pos.0, (self.ant_pos.1 + 1) % self.dimensions().1),
                    Direction::Left => (
                        if self.ant_pos.0 == 0 {
                            self.dimensions().0 - 1
                        } else {
                            self.ant_pos.0 - 1
                        },
                        self.ant_pos.1,
                    ),
                    Direction::Up => (
                        self.ant_pos.0,
                        if self.ant_pos.1 == 0 {
                            self.dimensions().1 - 1
                        } else {
                            self.ant_pos.1 - 1
                        },
                    ),
                };

                let color: Color;
                if let Cell::Color(new_c) = self.cells()[new_ant_pos.1][new_ant_pos.0] {
                    color = new_c
                } else {
                    panic!("Should always be a color.");
                }

                delta.push((new_ant_pos, Cell::Ant(new_direction, color)))
            } else {
                panic!("This should always be an ant.");
            }

            delta
        }

        fn delta(&self) -> &Vec<(Index, Cell)> {
            &self.delta
        }

        fn delta_mut(&mut self) -> &mut Vec<(Index, Cell)> {
            &mut self.delta
        }

        fn dimensions(&self) -> Dimensions {
            self.dimensions
        }

        fn tick(&mut self) {
            let delta = self.changes();
            assert_eq!(delta.len(), 2);
            let color = delta[0];
            let ant = delta[1];
            self.ant_pos = ant.0;
            self.cells_mut()[color.0 .1][color.0 .0] = color.1;
            self.cells_mut()[ant.0 .1][ant.0 .0] = ant.1;

            *self.delta_mut() = delta;
        }

        fn blank(&self) -> Self {
            let default = Cell::Color(Color {
                value: 0,
                cell_type: self.pattern[0],
            });
            Self::new_with_pattern(
                vec![vec![default; self.dimensions().0]; self.dimensions().1],
                self.dimensions(),
                self.pattern.clone(),
            )
        }

        fn random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self {
            let cells = vec![vec![(); self.dimensions().0]; self.dimensions().1]
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|_| Cell::random_with_pattern(rng, &self.pattern))
                        .collect()
                })
                .collect();

            Self::new_with_pattern(cells, self.dimensions(), self.pattern.clone())
        }
    }

    impl World {
        pub fn new_with_pattern(
            cells: DoubleVec<Cell>,
            dimensions: Dimensions,
            pattern: Vec<CellType>,
        ) -> Self {
            let mut w = Self::new(cells, dimensions);
            w.pattern = pattern;
            w
        }

        pub fn random_with_pattern_of_length<R: Rng + ?Sized>(
            rng: &mut R,
            dimensions: Dimensions,
            n: usize,
        ) -> Self {
            let mut pattern = Vec::with_capacity(n);
            for _ in 0..n {
                let f: f64 = rng.gen();
                if f < 0.5 {
                    pattern.push(CellType::CW)
                } else {
                    pattern.push(CellType::CCW)
                }
            }

            Self::random_with_pattern_of(rng, dimensions, pattern)
        }

        pub fn random_with_pattern_of<R: Rng + ?Sized>(
            rng: &mut R,
            dimensions: Dimensions,
            pattern: Vec<CellType>,
        ) -> Self {
            let mut cells = Vec::with_capacity(dimensions.1);
            for _ in 0..dimensions.1 {
                let mut row = Vec::with_capacity(dimensions.0);
                for _ in 0..dimensions.0 {
                    let v = rng.gen_range(0..pattern.len());
                    row.push(Cell::Color(Color {
                        value: v,
                        cell_type: pattern[v],
                    }))
                }
                cells.push(row);
            }

            Self::new_with_pattern(cells, dimensions, pattern)
        }
    }
}
