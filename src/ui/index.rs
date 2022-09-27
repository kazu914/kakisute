use super::app_interactor::{AppInteractor, Mode};
use super::display_data::DisplayData;
use super::terminal_manager::{TerminalManage, TerminalManager};
use crate::app::AppTrait;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

pub fn run_app(app: &mut dyn AppTrait) -> Result<()> {
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
    terminal_manager.draw_frame(DisplayData::new(app_interactor))?;

    if let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        handle_input(code, modifiers, terminal_manager, app_interactor)?;
    };
    Ok(())
}

fn handle_input(
    key_code: KeyCode,
    key_modifier: KeyModifiers,
    terminal_manager: &mut dyn TerminalManage,
    app_interactor: &mut AppInteractor,
) -> Result<()> {
    match app_interactor.get_mode() {
        Mode::Insert => match (key_code, key_modifier) {
            (KeyCode::Esc, KeyModifiers::NONE) => {
                app_interactor.clear_file_name();
                app_interactor.enter_normal_mode();
            }
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                app_interactor.push_to_file_name(c);
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                app_interactor.pop_from_file_name();
            }
            (KeyCode::Enter, KeyModifiers::NONE) => {
                terminal_manager.exit_app_screen()?;
                app_interactor.create_new_kakisute_with_file_name()?;
                terminal_manager.enter_app_screen()?;
                app_interactor.reload();
                app_interactor.clear_file_name();
            }
            _ => {}
        },
        Mode::Normal => match (key_code, key_modifier) {
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                terminal_manager.exit_app_screen()?;
                app_interactor.exit();
            }
            (KeyCode::Char('N'), KeyModifiers::SHIFT) => {
                app_interactor.enter_insert_mode();
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) => {
                app_interactor.select_next();
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) => {
                app_interactor.select_previous();
            }
            (KeyCode::Char('e'), KeyModifiers::NONE) => {
                if app_interactor.is_kakisute_selected() {
                    terminal_manager.exit_app_screen()?;
                    app_interactor.edit_kakisute()?;
                    terminal_manager.enter_app_screen()?;
                    terminal_manager.clear_app_screen()?;
                }
            }
            (KeyCode::Char('n'), KeyModifiers::NONE) => {
                terminal_manager.exit_app_screen()?;
                app_interactor.create_new_kakisute()?;
                terminal_manager.enter_app_screen()?;
                terminal_manager.clear_app_screen()?;
                app_interactor.reload();
            }
            (KeyCode::Char('d'), KeyModifiers::NONE) => {
                app_interactor.enter_delete_mode();
            }
            _ => {}
        },
        Mode::DeleteConfirm => match (key_code, key_modifier) {
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('n'), KeyModifiers::NONE) => {
                app_interactor.enter_normal_mode();
            }
            (KeyCode::Char('Y'), KeyModifiers::SHIFT) => {
                app_interactor.delete_kakisute()?;
                app_interactor.reload();
            }
            _ => {}
        },
    }
    Ok(())
}
