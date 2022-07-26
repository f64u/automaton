use cellular_automaton::{
    cell::BasicCell,
    common::{Grid, Index, RepresentableAs},
    space::{OutputField, Space},
    world::BasicWorld,
};

use crate::common::Output;

pub struct Html {
    pub value: String,
}

impl ToString for Html {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

pub trait WebCell: BasicCell + RepresentableAs<Html> {}

pub trait WebWorld<const W: usize, const H: usize>:
    BasicWorld<W, H> + RepresentableAs<Grid<Html, W, H>, Delta = Vec<(Index, Html)>>
where
    Self::Cell: WebCell,
{
    fn represent(&self) -> Grid<Html, W, H> {
        let mut grid = [[(); W]; H].map(|row| {
            row.map(|_| Html {
                value: String::new(),
            })
        });
        for (y, row) in self.cells().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                grid[y][x] = cell.represent()
            }
        }

        grid
    }
}

type OutputHtml<const W: usize, const H: usize> = Output<Grid<Html, W, H>>;

struct Browser<World, const W: usize, const H: usize>
where
    World: WebWorld<W, H>,
    World::Cell: WebCell,
{
    world: World,
    output: OutputHtml<W, H>,
}

impl<World, const W: usize, const H: usize> Space<World, OutputHtml<W, H>, Html, W, H>
    for Browser<World, W, H>
where
    World: WebWorld<W, H>,
    World::Cell: WebCell,
{
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn world(&self) -> &World {
        &self.world
    }

    fn output_mut(&mut self) -> &mut OutputHtml<W, H> {
        &mut self.output
    }
}

impl<World, const W: usize, const H: usize> Browser<World, W, H>
where
    World: WebWorld<W, H>,
    World::Cell: WebCell,
{
    fn new(world: World, output: OutputHtml<W, H>) -> Self {
        Self { world, output }
    }
}

impl<const W: usize, const H: usize> OutputField<W, H> for OutputHtml<W, H> {
    type Unit = Html;

    fn set_unit(&mut self, (x, y): Index, unit: Self::Unit, _refresh: bool) -> Result<(), String> {
        self.field[y][x] = unit;
        Ok(())
    }

    fn show(&mut self) {}
}
