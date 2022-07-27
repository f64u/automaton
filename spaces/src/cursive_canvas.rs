use std::{marker::PhantomData, time::Duration};

use cellular_automaton::{
    cell::BasicCell,
    common::{DoubleVec, Index},
    space::{OutputField, Space},
    world::BasicWorld,
};
use cursive::views::{LinearLayout, TextContent, TextView};

use crate::common::OutputManager;

type Out = OutputManager<DoubleVec<TextContent>>;

impl<C> OutputField<C, char> for Out
where
    C: BasicCell,
{
    fn set_unit(&mut self, (x, y): Index, unit: char, _refresh: bool) -> Result<(), String> {
        self.field[y][x].set_content(unit);
        Ok(())
    }
    fn show(&mut self) {}
}

struct Terminal<W, C>
where
    W: BasicWorld<C>,
    C: BasicCell,
{
    world: W,
    output: Out,
    _cell: PhantomData<C>,
}

impl<W, C> Space<W, C, Out> for Terminal<W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    type CellRepr = char;
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Out {
        &mut self.output
    }
}

impl<W, C> Terminal<W, C>
where
    W: BasicWorld<C>,
    C: BasicCell,
{
    fn new(world: W, output: Out) -> Self {
        Self {
            world,
            output,
            _cell: PhantomData,
        }
    }
}

pub fn run<W, C, F>(world: W, repr: F, update_millis: usize) -> Result<(), String>
where
    W: BasicWorld<C> + Send + 'static,
    C: BasicCell + Send + 'static,
    F: Fn(C) -> char + Send + 'static,
{
    let mut siv = cursive::default();
    siv.set_autorefresh(true);

    let texts: Vec<Vec<TextContent>> = vec![vec![(); world.dimensions().0]; world.dimensions().1]
        .into_iter()
        .map(|r| r.into_iter().map(|_| TextContent::new("")).collect())
        .collect();
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
        OutputManager {
            field: texts,
            pixel_size: 1,
        },
    );
    siv.add_global_callback('q', |s| s.quit());
    canvas.draw_whole(&repr)?;

    std::thread::spawn(move || loop {
        let _ = canvas.tick_delta(&repr);
        std::thread::sleep(Duration::from_millis(update_millis as u64));
    });

    siv.run();

    Ok(())
}
