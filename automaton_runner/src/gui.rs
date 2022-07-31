use briansbrain::{cell::Cell as BrainCell, world::World as BrainWorld};
use cellular_automaton::{common::Dimensions, world::BasicWorld};
use gameoflife::{cell::Cell as LifeCell, world::World as LifeWorld};
use langtonsant::{
    cell::{Cell as LangtonsCell, CellType, Color as CellColor},
    world::World as LangtonsWorld,
};
use sdl2::pixels::Color;
use spaces::sdl2_canvas::{self, Config};

use crate::worlds::Worlds;

pub fn run(
    world: Worlds,
    window_dimensions: Dimensions,
    pixel_size: usize,
    update_millis: usize,
) -> Result<(), String> {
    let world_dimenions = Dimensions(
        window_dimensions.0 / pixel_size,
        window_dimensions.1 / pixel_size,
    );

    let mut rng = rand::thread_rng();
    let config = Config::new(window_dimensions, pixel_size, update_millis as u64);

    match world {
        Worlds::GameOfLife => {
            let world = LifeWorld::new_random(&mut rng, world_dimenions);
            sdl2_canvas::run(config, world, "Game of Life", |c| match c {
                LifeCell::Alive => Color::WHITE,
                LifeCell::Dead => Color::BLACK,
            })?;
        }
        Worlds::BriansBrain => {
            let world = BrainWorld::new_random(&mut rng, world_dimenions);
            sdl2_canvas::run(config, world, "Brian's Brian", |c| match c {
                BrainCell::On => Color::WHITE,
                BrainCell::Dying => Color::BLUE,
                BrainCell::Off => Color::BLACK,
            })?;
        }
        Worlds::LangtonsAnt => {
            use CellType::*;
            let world = LangtonsWorld::random_with_pattern_of(
                &mut rng,
                world_dimenions,
                vec![CCW, CW, CW, CW, CW, CW, CCW, CCW, CW],
            );
            println!("{:?}", world.pattern);

            sdl2_canvas::run(config, world, "Langton's Ant", |c| {
                let colors = [
                    Color::BLACK,
                    Color::RGB(75, 0, 130),
                    Color::RGB(153, 50, 204),
                    Color::RGB(255, 0, 255),
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
