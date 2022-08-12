use auto_cellular::{common::Dimensions, world::BasicWorld};

use auto_spaces::cursive_canvas;

use auto_worlds::{
    briansbrain::{Cell as BrainCell, World as BrainWorld},
    gameoflife::{Cell as LifeCell, World as LifeWorld},
    langtonsant::{
        cell::{Cell as LangtonsCell, CellType, Color, Direction},
        world::World as LangtonsWorld,
    },
    Worlds,
};

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
            use CellType::*;
            let world = LangtonsWorld::random_with_pattern_of(
                &mut rng,
                dimensions,
                //vec![CCW, CW, CW, CW, CW, CW, CCW, CCW, CW],
                vec![CCW, CCW, CW, CW],
            );
            let world = world.blank();
            println!("{:?}", world.pattern);

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
