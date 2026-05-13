use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crate::renderer::ConsoleRenderer;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self, renderer: &mut ConsoleRenderer) -> Result<()> {
        while !self.should_quit {
            renderer.draw(|f| {
                let size = f.area();
                let block = Block::default()
                    .title("Srced IDE")
                    .borders(Borders::ALL);
                let paragraph = Paragraph::new("Welcome to Srced IDE! Press 'q' to quit.")
                    .block(block);
                f.render_widget(paragraph, size);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => self.should_quit = true,
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
