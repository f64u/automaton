mod game;
#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "gui")]
fn main() -> Result<(), String> {
    gui::run()?;

    Ok(())
}

#[cfg(not(feature = "gui"))]
fn main() -> Result<(), String> {
    Ok(())
}
