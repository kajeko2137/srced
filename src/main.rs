mod app;
mod renderer;

use crate::app::App;
use crate::renderer::ConsoleRenderer;
use anyhow::Result;

fn main() -> Result<()> {
    let mut renderer = ConsoleRenderer::new()?;
    let mut app = App::new();
    app.run(&mut renderer)?;
    Ok(())
}
