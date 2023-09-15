use std::time::{Duration, Instant};

use auto_cellular::{
    cell::CellLike,
    common::{Dimensions, Index},
    space::{OutputField, SpaceLike},
    world::WorldLike,
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

    pub fn downscale(&self, (x, y): (isize, isize)) -> Index {
        (x as usize / self.pixel_size, y as usize / self.pixel_size)
    }
}

type Out<'a> = OutputManager<&'a mut Canvas<Window>>;

impl<'a, C> OutputField<C, Color> for Out<'a>
where
    C: CellLike,
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

struct Gui<'a, W>
where
    W: WorldLike,
{
    world: W,
    output: Out<'a>,
    reprer: fn(W::Cell) -> Color,
}

impl<'a, W> SpaceLike<W, Out<'a>> for Gui<'a, W>
where
    W: WorldLike,
{
    type CellRepr = Color;
    type Reprer = fn(W::Cell) -> Color;
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

impl<'a, W> Gui<'a, W>
where
    W: WorldLike,
{
    fn new(world: W, output: Out<'a>, reprer: fn(W::Cell) -> Color) -> Self {
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

pub fn run<W>(
    config: Config,
    world: W,
    title: &str,
    repr: fn(W::Cell) -> Color,
) -> Result<(), String>
where
    W: WorldLike,
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
    let mut i = 0;
    let mut m: Duration = Duration::from_nanos(0);
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
                // Event::KeyDown {
                //     keycode: Some(Keycode::R),
                //     ..
                // } => gui.replace_with_random_world()?,
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
                } => gui.replace_with_blank_world()?,
                Event::MouseButtonDown { x, y, .. } => {
                    let (dx, dy) = config.downscale((x as isize, y as isize));
                    if is_paused {
                        gui.click_world((dx, dy));
                        gui.draw_whole()?
                    } else {
                        is_paused = true;
                    }
                }

                _ => {}
            }
        }

        if !is_paused {
            let now = Instant::now();
            gui.tick_whole()?;
            let d = now.elapsed();
            m = m.max(d);
            if i % 61 == 0 {
                println!("{:?}", now.elapsed());
            }

            i += 1;
        }

        std::thread::sleep(Duration::from_millis(millis));
    }

    println!("{:?}", m);

    Ok(())
}
