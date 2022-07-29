use cellular_automaton::{
    common::{linearize, Dimensions, DoubleVec, Index},
    world::BasicWorld,
};

use crate::cell::{Cell, CellType, CellVariant, Color, Direction};

pub struct World {
    cells: DoubleVec<Cell>,
    dimensions: Dimensions,
    delta: Vec<(Index, Cell)>,
    ant_pos: Index,
}

impl BasicWorld<Cell> for World {
    fn new(mut cells: DoubleVec<Cell>, dimensions: Dimensions) -> Self {
        let mut ant_pos = None;
        for (j, row) in cells.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                if matches!(
                    c,
                    Cell {
                        variant: CellVariant::Ant(_, _),
                        ..
                    },
                ) {
                    ant_pos = Some((i, j));
                }
            }
        }

        if ant_pos.is_none() {
            ant_pos = Some((dimensions.0 / 2, dimensions.1 / 2));
            let ant_pos = ant_pos.unwrap();
            cells[ant_pos.1][ant_pos.0] = Cell {
                pattern: &[CellType::CW, CellType::CCW],
                variant: CellVariant::Ant(
                    Direction::Right,
                    Color {
                        value: 0,
                        cell_type: CellType::CW,
                    },
                ),
            }
        }

        let delta = linearize(cells.clone());

        Self {
            cells,
            dimensions,
            delta,
            ant_pos: ant_pos.unwrap(),
        }
    }

    fn cells(&self) -> &DoubleVec<Cell> {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut DoubleVec<Cell> {
        &mut self.cells
    }

    fn changes(&self) -> Vec<(Index, Cell)> {
        let mut delta = vec![];
        let ant = self.cells()[self.ant_pos.1][self.ant_pos.0];
        if let Cell {
            pattern,
            variant: CellVariant::Ant(d, c),
        } = ant
        {
            let value = (c.value + 1) % pattern.len();
            delta.push((
                self.ant_pos,
                Cell {
                    pattern,
                    variant: CellVariant::Color(Color {
                        value,
                        cell_type: pattern[value],
                    }),
                },
            ));
            let new_ant_pos = match d {
                Direction::Right => ((self.ant_pos.0 + 1) % self.dimensions().0, self.ant_pos.1),
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

            let new_direction = match pattern[c.value] {
                CellType::CW => d.cw(),
                CellType::CCW => d.ccw(),
            };

            let color: Color;
            if let Cell {
                variant: CellVariant::Color(new_c),
                ..
            } = self.cells()[new_ant_pos.1][new_ant_pos.0]
            {
                color = new_c
            } else {
                panic!("Should always be a color.");
            }

            delta.push((
                new_ant_pos,
                Cell {
                    variant: CellVariant::Ant(new_direction, color),
                    pattern,
                },
            ))
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
}
