use cellular_automaton::{common::Dimensions, world::BasicWorld};
use spaces::cursive_canvas;

use briansbrain::{cell::Cell as BrainCell, world::World as BrainWorld};
use gameoflife::{cell::Cell as LifeCell, world::World as LifeWorld};

use crate::worlds::Worlds;

pub fn run(world: Worlds, dimensions: Dimensions, update_millis: usize) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    match world {
        Worlds::GameOfLife => {
            let world = LifeWorld::random(&mut rng, dimensions);

            cursive_canvas::run(
                world,
                |c| match c {
                    LifeCell::Alive => '#',
                    LifeCell::Dead => ' ',
                },
                update_millis,
            )?;
        }
        Worlds::BriansBrain => {
            let world = BrainWorld::random(&mut rng, dimensions);
            cursive_canvas::run(
                world,
                |c| match c {
                    BrainCell::On => 'O',
                    BrainCell::Dying => '*',
                    BrainCell::Off => ' ',
                },
                update_millis,
            )?;
        }
    }

    Ok(())
}
