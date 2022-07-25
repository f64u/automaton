use std::ops::DerefMut;

use cellular_automaton::{
    cell::BasicCell,
    common::Representable,
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

pub trait WebCell: BasicCell + Representable<Html> {}

pub trait WebWorld: BasicWorld + Representable<Html>
where
    Self::Cell: WebCell,
{
    fn represent(&self) -> Html {
        Html {
            value: format!(
                "<div class=\"world\">{}<div>",
                self.cells()
                    .chunks(self.dimensions().0)
                    .map(|chunk| {
                        format!(
                            "<div class=\"row\">{}</div>",
                            chunk
                                .iter()
                                .map(|c| Representable::<Html>::represent(c).to_string())
                                .collect::<String>()
                        )
                    })
                    .collect::<String>()
            ),
        }
    }
}

struct Browser<W>
where
    W: WebWorld,
    W::Cell: WebCell,
{
    world: W,
    output: Output<Html>,
}

impl<W> Space<W, Output<Html>, Html> for Browser<W>
where
    W: WebWorld,
    W::Cell: WebCell,
{
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Output<Html> {
        &mut self.output
    }
}

impl<W> Browser<W>
where
    W: WebWorld,
    W::Cell: WebCell,
{
    fn new(world: W, output: Output<Html>) -> Self {
        Self { world, output }
    }
}

impl OutputField for Output<Html> {
    type Data = Html;

    fn update(&mut self, data: Self::Data) -> Result<(), String> {
        self.value = data.to_string();
        Ok(())
    }
}
