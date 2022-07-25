use std::time::Duration;

use cellular_automaton::{
    cell::BasicCell,
    common::{Dimensions, Position, Representable},
    space::{OutputField, Space},
    world::BasicWorld,
};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::common::SizedOutput;

pub struct Config {
    pub dimensions: Dimensions,
    pub pixel_size: usize,
    pub millis: u64,
}

impl Config {
    pub fn new(dimensions: Dimensions, pixel_size: usize, millis: u64) -> Self {
        Self {
            dimensions,
            pixel_size,
            millis,
        }
    }

    pub fn pixel_count_x(&self) -> usize {
        self.dimensions.0 / self.pixel_size
    }

    pub fn pixel_count_y(&self) -> usize {
        self.dimensions.1 / self.pixel_size
    }

    pub fn downscale(&self, (x, y): (i32, i32)) -> Position {
        (
            x as isize / self.pixel_size as isize,
            y as isize / self.pixel_size as isize,
        )
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
    world: W,
    output: SizedOutput<&'a mut Canvas<Window>>,
}

impl<'a, W> Space<W, SizedOutput<&'a mut Canvas<Window>>, RepresentedWorld> for Gui<'a, W>
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

    fn output_mut(&mut self) -> &mut SizedOutput<&'a mut Canvas<Window>> {
        &mut self.output
    }
}

impl<'a, W> Gui<'a, W>
where
    W: ColoredWorld,
    W::Cell: ColoredCell,
{
    fn new(world: W, output: SizedOutput<&'a mut Canvas<Window>>) -> Self {
        Gui { world, output }
    }

    fn clear_output(&mut self) {
        self.output_mut().set_draw_color(Color::RGB(255, 255, 255));
        self.output_mut().clear();
    }
}

impl<'a> OutputField for SizedOutput<&'a mut Canvas<Window>> {
    type Data = RepresentedWorld;

    fn update(&mut self, data: Self::Data) -> Result<(), String> {
        for (index, cell) in data.0.iter().enumerate() {
            let (x, y) = data.1.get_pos(index);
            let rect = Rect::new(
                (x as usize * self.1) as i32,
                (y as usize * self.1) as i32,
                self.1 as u32,
                self.1 as u32,
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
    let mut millis = config.millis;

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
    let size = config.pixel_size;
    let mut gui = Gui::new(world, SizedOutput(&mut canvas, size));
    gui.clear_output();
    gui.draw()?;

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
                    gui.world_mut().refresh_random();
                    gui.draw()?;
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
                        gui.update()?;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    gui.world_mut().blank();
                    gui.draw()?;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    let (dx, dy) = config.downscale((x, y));
                    if is_paused {
                        if let Some(cell) = gui.world_mut().get_cell_mut((dx, dy)) {
                            cell.next();
                            gui.clear_output();
                            gui.draw()?;
                        }
                    }
                }

                _ => {}
            }
        }

        if !is_paused {
            gui.update()?;
        }

        std::thread::sleep(Duration::from_millis(millis));
        gui.clear_output();
    }

    Ok(())
}
