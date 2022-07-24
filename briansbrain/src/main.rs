mod game;
#[cfg(feature = "sdl2")]
mod gui;

#[cfg(feature = "sdl2")]
fn main() -> Result<(), String> {
    gui::run()?;

    Ok(())
}

#[cfg(not(feature = "sdl2"))]
fn main() -> Result<(), String> {
    Ok(())
}
