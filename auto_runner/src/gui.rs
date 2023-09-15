use auto_cellular::{common::Dimensions, world::WorldLike};
use auto_spaces::sdl2_canvas::{self, Config};
use auto_worlds::WorldKind;
use sdl2::pixels::Color;

pub fn run(
    world: WorldKind,
    window_dimensions: Dimensions,
    pixel_size: usize,
    update_millis: usize,
) -> Result<(), String> {
    let world_dimensions = Dimensions(
        window_dimensions.0 / pixel_size,
        window_dimensions.1 / pixel_size,
    );

    let mut rng = rand::thread_rng();
    let config = Config::new(window_dimensions, pixel_size, update_millis as u64);

    match world {
        WorldKind::GameOfLife => {
            use auto_worlds::gameoflife::{Cell, WConfig, World};
            let world_config = WConfig {
                dimensions: world_dimensions,
            };
            let world = World::new_random(&mut rng, world_config);
            sdl2_canvas::run(config, world, "Game of Life", |c| match c {
                Cell::Alive => Color::RGB(248, 90, 202),
                Cell::Dead => Color::RGB(16, 7, 32),
            })?;
        }
        WorldKind::BriansBrain => {
            use auto_worlds::briansbrain::{Cell, WConfig, World};
            let world_config = WConfig {
                dimensions: world_dimensions,
            };
            let world = World::new_random(&mut rng, world_config);
            sdl2_canvas::run(config, world, "Brian's Brian", |c| match c {
                Cell::On => Color::RGB(255, 229, 180),
                Cell::Dying => Color::RGB(31, 70, 144),
                Cell::Off => Color::RGB(35, 25, 85),
            })?;
        }
        WorldKind::LangtonsAnt => {
            use auto_worlds::langtonsant::{
                cell::{Cell, CellType, Color as CellColor},
                world::{WConfig, World},
            };
            use CellType::*;
            let world_config = WConfig {
                dimensions: world_dimensions,
            };
            let world =
                World::random_with_pattern_of(&mut rng, world_config, vec![CCW, CCW, CW, CW]);
            println!("{:?}", world.pattern);

            sdl2_canvas::run(config, world, "Langton's Ant", |c| {
                let colors = [
                    Color::RGB(16, 7, 32),
                    Color::RGB(68, 45, 124),
                    Color::RGB(99, 65, 180),
                    Color::RGB(218, 250, 139),
                    Color::RGB(230, 230, 250),
                    Color::RGB(32, 178, 170),
                    Color::RGB(152, 251, 152),
                    Color::RGB(50, 205, 50),
                    Color::RGB(0, 255, 0),
                    Color::WHITE,
                ];

                match c {
                    Cell::Ant(_, _) => Color::RED,
                    Cell::Color(CellColor { value, .. }) => colors[value],
                }
            })?;
        }
    }

    Ok(())
}
