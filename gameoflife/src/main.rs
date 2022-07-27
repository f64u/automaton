mod game;
#[cfg(feature = "sdl2")]
mod gui;
#[cfg(feature = "cursive")]
mod terminal;
#[cfg(feature = "wasm")]
mod web;

#[cfg(all(feature = "sdl2", feature = "cursive"))]
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() && args[0] == "in_terminal" {
        terminal::run()?;
        Ok(())
    } else {
        gui::run()?;
        Ok(())
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
