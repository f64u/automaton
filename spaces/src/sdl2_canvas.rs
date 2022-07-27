use std::time::Duration;

use cellular_automaton::{
    cell::BasicCell,
    common::{Dimensions, Index, ScreenPosition},
    space::{OutputField, Space},
    world::BasicWorld,
};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::common::OutputManager;

pub struct Config {
    pub dimensions: Dimensions, // dimensions of window
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
        self.dimensions.0 as usize / self.pixel_size
    }

    pub fn pixel_count_y(&self) -> usize {
        self.dimensions.1 as usize / self.pixel_size
    }

    pub fn downscale(&self, (x, y): ScreenPosition) -> Index {
        (x as usize / self.pixel_size, y as usize / self.pixel_size)
    }
}

type Out<'a> = OutputManager<&'a mut Canvas<Window>>;

impl<'a, C> OutputField<C, Color> for Out<'a>
where
    C: BasicCell,
{
    fn set_unit(&mut self, (x, y): Index, unit: Color, refresh: bool) -> Result<(), String> {
        let rect = Rect::new(
            (x * self.pixel_size) as i32,
            (y * self.pixel_size) as i32,
            self.pixel_size as u32,
            self.pixel_size as u32,
        );

        self.field.set_draw_color(unit);
        self.field.fill_rect(rect)?;

        if refresh {
            OutputField::<C, Color>::show(self)
        }

        Ok(())
    }

    fn show(&mut self) {
        self.field.present()
    }
}

struct Gui<'a, W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    world: W,
    output: Out<'a>,
    reprer: fn(C) -> Color,
}

impl<'a, W, C> Space<W, C, Out<'a>> for Gui<'a, W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    type CellRepr = Color;
    type Reprer = fn(C) -> Color;
    fn world_mut(&mut self) -> &mut W {
        &mut self.world
    }

    fn world(&self) -> &W {
        &self.world
    }

    fn output_mut(&mut self) -> &mut Out<'a> {
        &mut self.output
    }

    fn reprer(&self) -> Self::Reprer {
        self.reprer
    }
}

impl<'a, W, C> Gui<'a, W, C>
where
    C: BasicCell,
    W: BasicWorld<C>,
{
    fn new(world: W, output: Out<'a>, reprer: fn(C) -> Color) -> Self {
        Gui {
            world,
            output,
            reprer,
        }
    }

    fn clear_output(&mut self) {
        self.output_mut().field.set_draw_color(Color::WHITE);
        self.output_mut().field.clear();
        self.output_mut().field.present();
    }
}

pub fn run<W, C>(config: Config, world: W, title: &str, repr: fn(C) -> Color) -> Result<(), String>
where
    C: BasicCell,
    W: BasicWorld<C>,
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
        .position_centered()
        .resizable()
        .build()
        .map_err(|s| s.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|s| s.to_string())?;
    let output = OutputManager {
        field: &mut canvas,
        pixel_size: config.pixel_size,
    };

    let mut gui = Gui::new(world, output, repr);
    gui.clear_output();
    gui.draw_whole()?;

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
                    let d = gui.world().dimensions();
                    let mut rng = rand::thread_rng();
                    *gui.world_mut() = W::random(&mut rng, d);
                    gui.draw_whole()?;
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
                        gui.tick_whole()?
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    gui.world_mut().blank();
                    gui.draw_whole()?;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    let (dx, dy) = config.downscale((x as isize, y as isize));
                    if is_paused {
                        let cell = &mut gui.world_mut().cells_mut()[dy][dx];
                        *cell = cell.next_state();
                        gui.draw_whole()?
                    } else {
                        is_paused = true;
                    }
                }

                _ => {}
            }
        }

        if !is_paused {
            gui.tick_whole()?
        }

        std::thread::sleep(Duration::from_millis(millis));
    }

    Ok(())
}
