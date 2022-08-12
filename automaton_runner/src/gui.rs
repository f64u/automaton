use cellular_automaton::{common::Dimensions, world::BasicWorld};
use sdl2::pixels::Color;
use spaces::sdl2_canvas::{self, Config};
use worlds::{
    briansbrain::{Cell as BrainCell, World as BrainWorld},
    gameoflife::{Cell as LifeCell, World as LifeWorld},
    langtonsant::{
        cell::{Cell as LangtonsCell, CellType, Color as CellColor},
        world::World as LangtonsWorld,
    },
};

use worlds::Worlds;

pub fn run(
    world: Worlds,
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
        Worlds::GameOfLife => {
            let world = LifeWorld::new_random(&mut rng, world_dimensions);
            sdl2_canvas::run(config, world, "Game of Life", |c| match c {
                LifeCell::Alive => Color::RGB(248, 90, 202),
                LifeCell::Dead => Color::RGB(16, 7, 32),
            })?;
        }
        Worlds::BriansBrain => {
            let world = BrainWorld::new_random(&mut rng, world_dimensions);
            sdl2_canvas::run(config, world, "Brian's Brian", |c| match c {
                BrainCell::On => Color::RGB(255, 229, 180),
                BrainCell::Dying => Color::RGB(31, 70, 144),
                BrainCell::Off => Color::RGB(35, 25, 85),
            })?;
        }
        Worlds::LangtonsAnt => {
            use CellType::*;
            let world = LangtonsWorld::random_with_pattern_of(
                &mut rng,
                world_dimensions,
                vec![CCW, CCW, CW, CW],
            );
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
                    LangtonsCell::Ant(_, _) => Color::RED,
                    LangtonsCell::Color(CellColor { value, .. }) => colors[value],
                }
            })?;
        }
    }

    Ok(())
}
