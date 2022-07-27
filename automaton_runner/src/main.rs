#[cfg(feature = "sdl2")]
mod gui;
#[cfg(feature = "cursive")]
mod terminal;
#[cfg(feature = "wasm")]
mod web;

mod worlds;

#[cfg(all(feature = "sdl2", feature = "cursive"))]
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        return Err("don't work like that".into());
    }
    match (&args[0][..], &args[1][..]) {
        ("terminal", "gof") => terminal::run(worlds::Worlds::GameOfLife),
        ("terminal", "bb") => terminal::run(worlds::Worlds::BriansBrain),
        ("gui", "gof") => gui::run(worlds::Worlds::GameOfLife),
        ("gui", "bb") => gui::run(worlds::Worlds::BriansBrain),

        _ => {
            return Err("can't understand".into());
        }
    }
}

#[cfg(all(feature = "sdl2", not(feature = "cursive")))]
fn main() -> Result<(), String> {
    gui::run()?;
    Ok(())
}

#[cfg(all(feature = "cursive", not(feature = "sdl2")))]
fn main() -> Result<(), String> {
    terminal::run()?;
    Ok(())
}

#[cfg(not(any(feature = "cursive", feature = "sdl2")))]
fn main() {}
