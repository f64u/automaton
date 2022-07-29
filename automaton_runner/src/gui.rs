use briansbrain::{cell::Cell as BrainCell, world::World as BrainWorld};
use cellular_automaton::{common::Dimensions, world::BasicWorld};
use gameoflife::{cell::Cell as LifeCell, world::World as LifeWorld};
use langtonsant::{
    cell::{Cell as LangtonsCell, CellVariant, Color as CellColor},
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
            let world = LifeWorld::random(&mut rng, world_dimenions);
            sdl2_canvas::run(config, world, "Game of Life", |c| match c {
                LifeCell::Alive => Color::WHITE,
                LifeCell::Dead => Color::BLACK,
            })?;
        }
        Worlds::BriansBrain => {
            let world = BrainWorld::random(&mut rng, world_dimenions);
            sdl2_canvas::run(config, world, "Brian's Brian", |c| match c {
                BrainCell::On => Color::WHITE,
                BrainCell::Dying => Color::BLUE,
                BrainCell::Off => Color::BLACK,
            })?;
        }
        Worlds::LangtonsAnt => {
            let world = LangtonsWorld::random(&mut rng, world_dimenions);
            sdl2_canvas::run(config, world, "Langton's Ant", |c| {
                let colors = [Color::WHITE, Color::BLACK];
                match c {
                    LangtonsCell {
                        variant: CellVariant::Ant(_, _),
                        ..
                    } => Color::RED,
                    LangtonsCell {
                        variant: CellVariant::Color(CellColor { value, .. }),
                        ..
                    } => colors[value],
                }
            })?;
        }
    }

    Ok(())
}
