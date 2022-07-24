mod game;
#[cfg(feature = "gui")]
mod gui;
mod terminal;

#[cfg(feature = "gui")]
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

#[cfg(not(feature = "gui"))]
fn main() -> Result<(), String> {
    terminal::run()?;
    Ok(())
}
