use cellular_automaton::{common::Dimensions, world::BasicWorld};

use spaces::cursive_canvas;

use briansbrain::{cell::Cell as BrainCell, world::World as BrainWorld};
use gameoflife::{cell::Cell as LifeCell, world::World as LifeWorld};
use langtonsant::{
    cell::{Cell as LangtonsCell, Color, Direction},
    world::World as LangtonsWorld,
};

use crate::worlds::Worlds;

pub fn run(world: Worlds, dimensions: Dimensions, update_millis: usize) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    match world {
        Worlds::GameOfLife => {
            let world = LifeWorld::new_random(&mut rng, dimensions);

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
            let world = BrainWorld::new_random(&mut rng, dimensions);
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
        Worlds::LangtonsAnt => {
            let mut rng = rand::thread_rng();
            let world = LangtonsWorld::new_random(&mut rng, dimensions);

            cursive_canvas::run(
                world,
                |c| {
                    let colors = "!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~";
                    match c {
                        LangtonsCell::Ant(d, _) => match d {
                            Direction::Left => '⇦',
                            Direction::Right => '⇨',
                            Direction::Up => '⇧',
                            Direction::Down => '⇩',
                        },
                        LangtonsCell::Color(Color { value, .. }) => {
                            colors.chars().nth(value).unwrap_or('?')
                        }
                    }
                },
                update_millis,
            )?;
        }
    }

    Ok(())
}
