use super::app_interactor::{AppInteractor, Mode};

use super::renderer::{HELP_BOX_LENGTH, MARGIN};
use super::terminal_manager::TerminalManage;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};

pub fn handle_input(
    key_code: KeyCode,
    key_modifier: KeyModifiers,
    terminal_manager: &mut dyn TerminalManage,
    app_interactor: &mut AppInteractor,
) -> Result<()> {
    match app_interactor.get_mode() {
        Mode::Insert => match (key_code, key_modifier) {
            (KeyCode::Esc, KeyModifiers::NONE) => {
                app_interactor.clear_user_input();
                app_interactor.enter_mode(Mode::Normal);
            }
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                app_interactor.push_user_input(c);
            }
            (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                app_interactor.push_user_input(c);
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                app_interactor.pop_user_input();
            }
            (KeyCode::Enter, KeyModifiers::NONE) => {
                terminal_manager.exit_app_screen()?;
                app_interactor.create_new_kakisute_with_file_name()?;
                terminal_manager.enter_app_screen()?;
                app_interactor.reload()?;
                app_interactor.clear_user_input();
            }
            _ => {}
        },
        Mode::Normal => match (key_code, key_modifier) {
            (KeyCode::Char('q'), KeyModifiers::NONE) => {
                terminal_manager.exit_app_screen()?;
                app_interactor.exit();
            }
            (KeyCode::Char('N'), KeyModifiers::SHIFT) => {
                app_interactor.enter_mode(Mode::Insert);
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) | (KeyCode::Down, KeyModifiers::NONE) => {
                app_interactor.select_next();
            }
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                let terminal_height = terminal_manager.get_terminal_height()?;
                let list_height = terminal_height - HELP_BOX_LENGTH - MARGIN * 4;
                app_interactor.select_previous_n(list_height / 2);
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) | (KeyCode::Up, KeyModifiers::NONE) => {
                app_interactor.select_previous();
            }
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                let terminal_height = terminal_manager.get_terminal_height()?;
                let list_height = terminal_height - HELP_BOX_LENGTH - MARGIN * 4;
                app_interactor.select_next_n(list_height / 2);
            }
            (KeyCode::Char('e'), KeyModifiers::NONE) => {
                if app_interactor.is_kakisute_selected() {
                    terminal_manager.exit_app_screen()?;
                    app_interactor.edit_kakisute()?;
                    terminal_manager.enter_app_screen()?;
                    terminal_manager.clear_app_screen()?;
                    app_interactor.reload()?;
                }
            }
            (KeyCode::Char('n'), KeyModifiers::NONE) => {
                terminal_manager.exit_app_screen()?;
                app_interactor.create_new_kakisute()?;
                terminal_manager.enter_app_screen()?;
                terminal_manager.clear_app_screen()?;
                app_interactor.reload()?;
            }
            (KeyCode::Char('d'), KeyModifiers::NONE) => {
                app_interactor.enter_mode(Mode::DeleteConfirm);
            }
            (KeyCode::Char('/'), KeyModifiers::NONE) => {
                app_interactor.enter_mode(Mode::Search);
            }
            _ => {}
        },
        Mode::DeleteConfirm => match (key_code, key_modifier) {
            (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('n'), KeyModifiers::NONE) => {
                app_interactor.enter_mode(Mode::Normal);
            }
            (KeyCode::Char('Y'), KeyModifiers::SHIFT) => {
                app_interactor.delete_kakisute()?;
                app_interactor.reload()?;
            }
            _ => {}
        },
        Mode::Search => match (key_code, key_modifier) {
            (KeyCode::Enter, KeyModifiers::NONE) => {
                app_interactor.enter_mode(Mode::Normal);
            }
            (KeyCode::Esc, KeyModifiers::NONE) => {
                app_interactor.clear_user_input();
                app_interactor.filter()?;
                app_interactor.enter_mode(Mode::Normal);
            }
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                app_interactor.push_user_input(c);
                app_interactor.filter()?
            }
            (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                app_interactor.push_user_input(c);
                app_interactor.filter()?
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                app_interactor.pop_user_input();
                app_interactor.filter()?
            }
            _ => {}
        },
    }
    Ok(())
}
