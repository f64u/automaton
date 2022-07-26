use std::time::Duration;

use cellular_automaton::{
    cell::BasicCell,
    common::{Grid, Index, Position, RepresentableAs},
    space::{OutputField, Space},
    world::BasicWorld,
};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::common::Output;

pub struct Config {
    pub dimensions: (u32, u32), // dimensions of window
    pub pixel_size: usize,
    pub millis: u64,
}

impl Config {
    pub fn new(dimensions: (u32, u32), pixel_size: usize, millis: u64) -> Self {
        Self {
            dimensions,
            pixel_size,
            millis,
        }
    }

    pub fn pixel_count_x(&self) -> usize {
        self.dimensions.0 as usize / self.pixel_size
    }

    pub fn pixel_count_y(&self) -> usize {
        self.dimensions.1 as usize / self.pixel_size
    }

    pub fn downscale(&self, (x, y): Position) -> Index {
        (x as usize / self.pixel_size, y as usize / self.pixel_size)
    }
}

pub trait ColoredCell: BasicCell + RepresentableAs<Color, Delta = Color> {
    fn next_frame(&self) -> Self::Delta {
        RepresentableAs::<Self::Delta>::represent(self)
    }
}

pub type RepresentedWorld<const W: usize, const H: usize> = Grid<Color, W, H>;
pub type FutureWorld = Vec<(Index, Color)>;

pub trait ColoredWorld<const W: usize, const H: usize>:
    BasicWorld<W, H> + RepresentableAs<RepresentedWorld<W, H>, Delta = FutureWorld>
where
    Self::Cell: ColoredCell,
{
    fn represent(&self) -> RepresentedWorld<W, H> {
        let mut grid = [[Color::WHITE; W]; H];
        for (y, row) in self.cells().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                grid[y][x] = cell.represent()
            }
        }

        grid
    }

    fn next_frame(&self) -> FutureWorld {
        self.delta_future()
            .into_iter()
            .map(|(index, cell)| (index, cell.represent()))
            .collect()
    }
}

struct Gui<'a, World, const W: usize, const H: usize>
where
    World: ColoredWorld<W, H>,
    World::Cell: ColoredCell,
{
    world: World,
    output: Output<&'a mut Canvas<Window>>,
}

impl<'a, World, const W: usize, const H: usize>
    Space<World, Output<&'a mut Canvas<Window>>, Color, W, H> for Gui<'a, World, W, H>
where
    World: ColoredWorld<W, H> + RepresentableAs<RepresentedWorld<W, H>>,
    World::Cell: ColoredCell,
{
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn world(&self) -> &World {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Output<&'a mut Canvas<Window>> {
        &mut self.output
    }
}

impl<'a, World, const W: usize, const H: usize> Gui<'a, World, W, H>
where
    World: ColoredWorld<W, H>,
    World::Cell: ColoredCell,
{
    fn new(world: World, output: Output<&'a mut Canvas<Window>>) -> Self {
        Gui { world, output }
    }

    fn clear_output(&mut self) {
        self.output_mut().field.set_draw_color(Color::WHITE);
        self.output_mut().field.clear();
        self.output_mut().field.present();
    }
}

impl<'a, const W: usize, const H: usize> OutputField<W, H> for Output<&'a mut Canvas<Window>> {
    type Unit = Color;

    fn set_unit(&mut self, (x, y): Index, unit: Self::Unit, refresh: bool) -> Result<(), String> {
        let rect = Rect::new(
            (x * self.pixel_size) as i32,
            (y * self.pixel_size) as i32,
            self.pixel_size as u32,
            self.pixel_size as u32,
        );

        self.field.set_draw_color(unit);
        self.field.fill_rect(rect)?;

        if refresh {
            OutputField::<W, H>::show(self)
        }

        Ok(())
    }

    fn show(&mut self) {
        self.field.present()
    }
}

pub fn run<World, const W: usize, const H: usize>(
    config: Config,
    world: World,
    title: &str,
) -> Result<(), String>
where
    World: ColoredWorld<W, H>,
    World::Cell: ColoredCell,
{
    assert_eq!(config.pixel_count_x(), W);
    assert_eq!(config.pixel_count_y(), H);

    let mut millis = config.millis;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            title,
            config.dimensions.0 as u32,
            config.dimensions.1 as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .map_err(|s| s.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|s| s.to_string())?;
    let output = Output {
        field: &mut canvas,
        pixel_size: config.pixel_size,
    };

    let mut gui = Gui::new(world, output);
    gui.clear_output();
    gui.draw_whole_world()?;

    let mut event_dump = sdl_context.event_pump()?;

    let mut is_paused = true;

    'running: loop {
        for event in event_dump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    gui.clear_output();
                    gui.world_mut().refresh_random();
                    gui.draw_whole_world()?;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    is_paused = !is_paused;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    millis = 10.max(millis - 10);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    millis += 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    if is_paused {
                        gui.world_mut().tick();
                        gui.draw_whole_world()?;
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    gui.world_mut().blank();
                    gui.draw_whole_world()?;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    let (dx, dy) = config.downscale((x as isize, y as isize));
                    if is_paused {
                        let cell = &mut gui.world_mut().cells_mut()[dy][dx];
                        *cell = cell.next();
                        gui.draw_whole_world()?;
                    } else {
                        is_paused = true;
                    }
                }

                _ => {}
            }
        }

        if !is_paused {
            gui.world_mut().tick();
            gui.draw_whole_world()?;
        }

        std::thread::sleep(Duration::from_millis(millis));
    }

    Ok(())
}
