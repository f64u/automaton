mod game;
mod gui;

fn main() -> Result<(), String> {
    gui::run()?;

    Ok(())
}
