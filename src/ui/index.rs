use super::app_interactor::AppInteractor;
use super::display_data::DisplayData;
use super::input_handler;

use super::terminal_manager::{TerminalManage, TerminalManager};
use crate::service::ServiceTrait;
use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

pub fn run_app(app: &mut dyn ServiceTrait) -> Result<()> {
    let mut app_interactor = AppInteractor::new(app);
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut terminal_manager = TerminalManager::new(terminal);
    terminal_manager.enter_app_screen()?;

    while !app_interactor.is_exited() {
        render_loop(&mut terminal_manager, &mut app_interactor)?
    }
    Ok(())
}

fn render_loop(
    terminal_manager: &mut dyn TerminalManage,
    app_interactor: &mut AppInteractor,
) -> Result<()> {
    terminal_manager.draw_frame(DisplayData::new(app_interactor.generate_info()))?;

    if let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        input_handler::handle_input(code, modifiers, terminal_manager, app_interactor)?;
    };
    Ok(())
}
