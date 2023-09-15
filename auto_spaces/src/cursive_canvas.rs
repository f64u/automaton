use std::time::Duration;

use auto_cellular::{
    cell::CellLike,
    common::{DoubleVec, Index},
    space::{OutputField, SpaceLike},
    world::{WorldConfig, WorldLike},
};
use cursive::views::{LinearLayout, TextContent, TextView};

use crate::common::OutputManager;

type Out = OutputManager<DoubleVec<TextContent>>;

impl<C> OutputField<C, char> for Out
where
    C: CellLike,
{
    fn set_unit(&mut self, (x, y): Index, unit: char, _refresh: bool) -> Result<(), String> {
        self.field[y][x].set_content(unit);
        Ok(())
    }
    fn show(&mut self) {}
}

struct Terminal<W>
where
    W: WorldLike,
{
    world: W,
    output: Out,
    reprer: fn(W::Cell) -> char,
}

impl<W> SpaceLike<W, Out> for Terminal<W>
where
    W: WorldLike,
{
    type CellRepr = char;
    type Reprer = fn(W::Cell) -> char;
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Out {
        &mut self.output
    }

    fn reprer(&self) -> Self::Reprer {
        self.reprer
    }
}

impl<W> Terminal<W>
where
    W: WorldLike,
{
    fn new(world: W, output: Out, reprer: <Self as SpaceLike<W, Out>>::Reprer) -> Self {
        Self {
            world,
            output,
            reprer,
        }
    }
}

pub fn run<W>(world: W, repr: fn(W::Cell) -> char, update_millis: usize) -> Result<(), String>
where
    W: WorldLike + Send + 'static,
{
    let mut siv = cursive::default();
    siv.set_autorefresh(true);

    let texts: Vec<Vec<TextContent>> =
        vec![vec![(); world.config().dimensions().0]; world.config().dimensions().1]
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
        repr,
    );
    siv.add_global_callback('q', |s| s.quit());
    canvas.draw_whole()?;

    std::thread::spawn(move || loop {
        let _ = canvas.tick_delta();
        std::thread::sleep(Duration::from_millis(update_millis as u64));
    });

    siv.run();

    Ok(())
}
