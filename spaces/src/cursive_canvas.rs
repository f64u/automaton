use std::time::Duration;

use cellular_automaton::{
    cell::BasicCell,
    common::{Grid, Index, RepresentableAs},
    space::{OutputField, Space},
    world::BasicWorld,
};
use cursive::views::{LinearLayout, TextContent, TextView};

use crate::common::Output;

pub trait CharCell: BasicCell + RepresentableAs<char> {}

pub trait StringWorld<const W: usize, const H: usize>:
    BasicWorld<W, H> + RepresentableAs<Grid<char, W, H>, Delta = Vec<(Index, char)>>
where
    Self::Cell: CharCell,
{
    fn represent(&self) -> Grid<char, W, H> {
        let mut grid = [[char::default(); W]; H];
        for (y, row) in self.cells().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                grid[y][x] = cell.represent()
            }
        }

        grid
    }
}

pub type OutputContent<const W: usize, const H: usize> = Output<Grid<TextContent, W, H>>;

struct Terminal<World, const W: usize, const H: usize>
where
    World: StringWorld<W, H>,
    World::Cell: CharCell,
{
    world: World,
    output: OutputContent<W, H>,
}

impl<World, const W: usize, const H: usize> Space<World, OutputContent<W, H>, char, W, H>
    for Terminal<World, W, H>
where
    World: StringWorld<W, H, Delta = Vec<(Index, char)>>,
    World::Cell: CharCell,
{
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn world(&self) -> &World {
        &self.world
    }

    fn output_mut(&mut self) -> &mut OutputContent<W, H> {
        &mut self.output
    }
}

impl<'a, World, const W: usize, const H: usize> Terminal<World, W, H>
where
    World: StringWorld<W, H>,
    World::Cell: CharCell,
{
    fn new(world: World, output: OutputContent<W, H>) -> Self {
        Self { world, output }
    }
}

impl<const W: usize, const H: usize> OutputField<W, H> for OutputContent<W, H> {
    type Unit = char;

    fn set_unit(&mut self, (x, y): Index, unit: Self::Unit, _refresh: bool) -> Result<(), String> {
        self.field[y][x].set_content(unit);
        Ok(())
    }
    fn show(&mut self) {}
}

pub fn run<World, const W: usize, const H: usize>(world: World) -> Result<(), String>
where
    World: StringWorld<W, H> + std::marker::Send + 'static,
    World::Cell: CharCell,
{
    let mut siv = cursive::default();
    siv.set_autorefresh(true);

    let texts = [[(); W]; H].map(|r| r.map(|_| TextContent::new("")));
    let textboxes = texts.iter().map(|row| {
        let mut layout_row = LinearLayout::horizontal();
        for child in row
            .iter()
            .map(|text| TextView::new_with_content(text.clone()))
        {
            layout_row.add_child(child)
        }
        layout_row
    });
    let mut layout = LinearLayout::vertical();
    for textbox in textboxes {
        layout.add_child(textbox);
    }
    siv.add_layer(layout);

    let mut canvas = Terminal::new(
        world,
        Output {
            field: texts,
            pixel_size: 1,
        },
    );
    siv.add_global_callback('q', |s| s.quit());
    canvas.draw_whole_world();

    std::thread::spawn(move || loop {
        let _ = canvas.draw_delta();
        canvas.world_mut().tick();
        std::thread::sleep(Duration::from_millis(100));
    });

    siv.run();

    Ok(())
}
