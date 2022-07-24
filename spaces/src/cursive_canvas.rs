use std::time::Duration;

use cellular_automaton::{
    cell::BasicCell,
    common::Representable,
    space::{OutputField, Space},
    world::BasicWorld,
};
use cursive::views::{TextContent, TextView};

use crate::common::Output;

pub trait StringCell: BasicCell + Representable<String> {}

pub trait StringWorld: BasicWorld + Representable<String>
where
    Self::Cell: StringCell,
{
    fn represent(&self) -> String {
        self.cells()
            .chunks(self.dimensions().0)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| Representable::represent(c))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

struct Terminal<W>
where
    W: StringWorld,
    W::Cell: StringCell,
{
    world: W,
    output: Output<TextContent>,
}

impl<W> Space<W, Output<TextContent>, String> for Terminal<W>
where
    W: StringWorld,
    W::Cell: StringCell,
{
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Output<TextContent> {
        &mut self.output
    }
}

impl<W> Terminal<W>
where
    W: StringWorld,
    W::Cell: StringCell,
{
    fn new(world: W, output: Output<TextContent>) -> Self {
        Self { world, output }
    }
}

impl OutputField for Output<TextContent> {
    type Data = String;

    fn update(&mut self, data: Self::Data) -> Result<(), String> {
        self.set_content(data);
        Ok(())
    }
}

pub fn run<W>(world: W) -> Result<(), String>
where
    W: StringWorld + std::marker::Send + 'static,
    W::Cell: StringCell,
{
    let mut siv = cursive::default();
    siv.set_autorefresh(true);

    let text = TextContent::new("");
    let textbox = TextView::new_with_content(text.clone());
    siv.add_layer(textbox);

    let mut canvas = Terminal::new(world, Output(text));
    siv.add_global_callback('q', |s| s.quit());

    std::thread::spawn(move || loop {
        canvas.update();
        std::thread::sleep(Duration::from_millis(100));
    });

    siv.run();

    Ok(())
}
