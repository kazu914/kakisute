use crate::ui::display_data::DisplayData;

use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::Backend, Terminal};

use anyhow::Result;

use super::renderer::render;

pub struct TerminalManager<B: Backend> {
    terminal: Terminal<B>,
}

impl<B: Backend> TerminalManager<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }
}

impl<B: Backend> TerminalManage for TerminalManager<B> {
    fn enter_app_screen(&mut self) -> Result<()> {
        enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    fn draw_frame(&mut self, display_data: DisplayData) -> Result<()> {
        self.terminal.draw(|f| render(f, display_data))?;
        Ok(())
    }
    fn exit_app_screen(&mut self) -> Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    fn clear_app_screen(&mut self) -> Result<()> {
        self.terminal.clear()?;
        Ok(())
    }
}

pub trait TerminalManage {
    fn enter_app_screen(&mut self) -> Result<()>;
    fn draw_frame(&mut self, display_data: DisplayData) -> Result<()>;
    fn exit_app_screen(&mut self) -> Result<()>;
    fn clear_app_screen(&mut self) -> Result<()>;
}
