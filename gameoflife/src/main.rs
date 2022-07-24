mod game;
#[cfg(feature = "sdl2")]
mod gui;
mod terminal;

#[cfg(feature = "sdl2")]
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() >= 1 && args[0] == "in_terminal" {
        terminal::run()?;
        Ok(())
    } else {
        gui::run()?;
        Ok(())
    }
}

#[cfg(not(feature = "sdl2"))]
fn main() -> Result<(), String> {
    terminal::run()?;
    Ok(())
}
