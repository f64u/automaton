use std::time::Duration;

const PIXEL_SIZE: usize = 5;

use cellular_automaton::{
    cell::BasicCell,
    common::{Dimensions, Representable},
    space::{OutputField, Space},
    world::BasicWorld,
};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::common::Output;

pub struct Config {
    pub dimensions: Dimensions,
    pub pixel_size: usize,
}

impl Config {
    pub fn new(dimensions: Dimensions, pixel_size: usize) -> Self {
        Self {
            dimensions,
            pixel_size,
        }
    }

    fn pixel_count_x(&self) -> usize {
        self.dimensions.0 / self.pixel_size
    }

    fn pixel_count_y(&self) -> usize {
        self.dimensions.1 / self.pixel_size
    }
}

pub trait ColoredCell: BasicCell + Representable<Color> {}

pub type RepresentedWorld = (Vec<Color>, Dimensions);

pub trait ColoredWorld: BasicWorld + Representable<RepresentedWorld>
where
    Self::Cell: ColoredCell,
{
    fn represent(&self) -> RepresentedWorld {
        (
            self.cells()
                .iter()
                .map(|cell| Representable::<Color>::represent(cell))
                .collect(),
            self.dimensions(),
        )
    }
}

struct Gui<'a, W>
where
    W: ColoredWorld,
    W::Cell: ColoredCell,
{
    config: Config,
    world: W,
    output: Output<&'a mut Canvas<Window>>,
}

impl<'a, W> Space<W, Output<&'a mut Canvas<Window>>, RepresentedWorld> for Gui<'a, W>
where
    W: ColoredWorld,
    W::Cell: ColoredCell,
{
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Output<&'a mut Canvas<Window>> {
        &mut self.output
    }
}

impl<'a, W> Gui<'a, W>
where
    W: ColoredWorld,
    W::Cell: ColoredCell,
{
    fn new(config: Config, world: W, output: Output<&'a mut Canvas<Window>>) -> Self {
        Gui {
            config,
            world,
            output,
        }
    }

    fn clear_output(&mut self) {
        self.output_mut().set_draw_color(Color::RGB(255, 255, 255));
        self.output_mut().clear();
    }
}

impl<'a> OutputField for Output<&'a mut Canvas<Window>> {
    type Data = RepresentedWorld;

    fn update(&mut self, data: Self::Data) -> Result<(), String> {
        for (index, cell) in data.0.iter().enumerate() {
            let (x, y) = data.1.get_pos(index);
            let rect = Rect::new(
                (x as usize * PIXEL_SIZE) as i32,
                (y as usize * PIXEL_SIZE) as i32,
                PIXEL_SIZE as u32,
                PIXEL_SIZE as u32,
            );

            self.set_draw_color(*cell);

            self.fill_rect(rect)?;
        }
        self.present();
        Ok(())
    }
}

pub fn run<W>(config: Config, world: W, title: &str) -> Result<(), String>
where
    W: ColoredWorld,
    W::Cell: ColoredCell,
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            title,
            config.dimensions.0 as u32,
            config.dimensions.1 as u32,
        )
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .map_err(|s| s.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|s| s.to_string())?;
    let mut gui = Gui::new(config, world, Output(&mut canvas));
    gui.clear_output();

    let mut event_dump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_dump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    gui.world_mut().refresh_random();
                }
                Event::MouseMotion { .. } => {}
                e => {
                    println!("{e:?}");
                }
            }
        }

        gui.update()?;

        std::thread::sleep(Duration::from_millis(50));
        gui.clear_output();
    }

    Ok(())
}
