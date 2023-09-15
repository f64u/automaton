use auto_cellular::{common::Dimensions, world::WorldLike};

use auto_spaces::cursive_canvas;

use auto_worlds::WorldKind;

pub fn run(world: WorldKind, dimensions: Dimensions, update_millis: usize) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    match world {
        WorldKind::GameOfLife => {
            use auto_worlds::gameoflife::{Cell, WConfig, World};
            let world = World::new_random(&mut rng, WConfig { dimensions });

            cursive_canvas::run(
                world,
                |c| match c {
                    Cell::Alive => '#',
                    Cell::Dead => ' ',
                },
                update_millis,
            )?;
        }
        WorldKind::BriansBrain => {
            use auto_worlds::briansbrain::{Cell, WConfig, World};
            let world = World::new_random(&mut rng, WConfig { dimensions });
            cursive_canvas::run(
                world,
                |c| match c {
                    Cell::On => 'O',
                    Cell::Dying => '*',
                    Cell::Off => ' ',
                },
                update_millis,
            )?;
        }
        WorldKind::LangtonsAnt => {
            use auto_worlds::langtonsant::{
                cell::{Cell, CellType::*, Color, Direction},
                world::{WConfig, World},
            };

            let world = World::random_with_pattern_of(
                &mut rng,
                WConfig { dimensions },
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
                        Cell::Ant(d, _) => match d {
                            Direction::Left => '⇦',
                            Direction::Right => '⇨',
                            Direction::Up => '⇧',
                            Direction::Down => '⇩',
                        },
                        Cell::Color(Color { value, .. }) => {
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
