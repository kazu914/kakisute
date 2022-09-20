use crate::app::App;
use crate::ui::display_data::DisplayData;
use crate::ui::tui_app::Tui;
use tui::{
    layout::{Alignment, Rect},
    widgets::Clear,
};
use unicode_width::UnicodeWidthStr;

use anyhow::{Ok, Result};
use std::io::{self, Stdout};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use super::tui_app::Mode;

pub fn run_app(app: &mut App) -> Result<()> {
    let kakisute_list = app.get_kakisute_list();
    let mut tui = Tui::new(kakisute_list);
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal
        .backend_mut()
        .execute(EnterAlternateScreen)?
        .execute(EnableMouseCapture)?;
    while !tui.exit {
        render_loop(&mut terminal, app, &mut tui)?
    }
    Ok(())
}

fn render_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    tui: &mut Tui,
) -> Result<()> {
    terminal.draw(|f| {
        render(f, DisplayData::new(app, tui));
    })?;

    if let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        match tui.mode {
            Mode::Insert => match (code, modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    tui.clear_file_name();
                    tui.enter_normal_mode();
                }
                (KeyCode::Char(c), KeyModifiers::NONE) => {
                    tui.new_file_name.push(c);
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    tui.new_file_name.pop();
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    terminal
                        .backend_mut()
                        .execute(LeaveAlternateScreen)?
                        .execute(DisableMouseCapture)?;
                    tui.create_new_kakisute_with_file_name(app)?;
                    terminal
                        .backend_mut()
                        .execute(EnterAlternateScreen)?
                        .execute(EnableMouseCapture)?;
                    terminal.clear()?;
                    app.reload();
                    tui.reload(app.get_kakisute_list());
                    tui.clear_file_name();
                }
                _ => {}
            },
            Mode::Normal => match (code, modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                    disable_raw_mode()?;
                    terminal
                        .backend_mut()
                        .execute(LeaveAlternateScreen)?
                        .execute(DisableMouseCapture)?;
                    terminal.show_cursor()?;
                    tui.exit = true;
                }
                (KeyCode::Char('N'), KeyModifiers::SHIFT) => {
                    tui.enter_insert_mode();
                }
                (KeyCode::Char('j'), KeyModifiers::NONE) => {
                    tui.select_next();
                }
                (KeyCode::Char('k'), KeyModifiers::NONE) => {
                    tui.select_previous();
                }
                (KeyCode::Char('e'), KeyModifiers::NONE) => {
                    if tui.selected_list_index.is_some() {
                        terminal
                            .backend_mut()
                            .execute(LeaveAlternateScreen)?
                            .execute(DisableMouseCapture)?;
                        tui.edit_kakisute(app)?;
                        terminal
                            .backend_mut()
                            .execute(EnterAlternateScreen)?
                            .execute(EnableMouseCapture)?;
                        terminal.clear()?;
                    }
                }
                (KeyCode::Char('n'), KeyModifiers::NONE) => {
                    terminal
                        .backend_mut()
                        .execute(LeaveAlternateScreen)?
                        .execute(DisableMouseCapture)?;
                    tui.create_new_kakisute(app)?;
                    terminal
                        .backend_mut()
                        .execute(EnterAlternateScreen)?
                        .execute(EnableMouseCapture)?;
                    terminal.clear()?;
                    app.reload();
                    tui.reload(app.get_kakisute_list());
                }
                (KeyCode::Char('d'), KeyModifiers::NONE) => {
                    tui.enter_delete_mode();
                }
                _ => {}
            },
            Mode::DeleteConfirm => match (code, modifiers) {
                (KeyCode::Esc, KeyModifiers::NONE) | (KeyCode::Char('n'), KeyModifiers::NONE) => {
                    tui.enter_normal_mode();
                }
                (KeyCode::Char('Y'), KeyModifiers::SHIFT) => {
                    if let Some(index) = tui.selected_list_index {
                        app.delete_by_index(index)?;
                        app.reload();
                        tui.reload(app.get_kakisute_list());
                    }
                }
                _ => {}
            },
        }
    };
    Ok(())
}

fn render<B: Backend>(f: &mut Frame<B>, display_data: DisplayData) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(f.size());

    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[0]);

    let file_names = display_data
        .kakisute_list
        .body
        .iter()
        .map(|file| ListItem::new(file.file_name()))
        .collect::<Vec<ListItem>>();

    let list = List::new(file_names)
        .block(
            Block::default()
                .title(display_data.kakisute_list.title)
                .borders(Borders::ALL)
                .border_style(match display_data.mode {
                    Mode::Normal => Style::default().fg(Color::Blue),
                    _ => Style::default(),
                }),
        )
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    let mut state = ListState::default();
    state.select(display_data.index);

    f.render_stateful_widget(list, content_chunk[0], &mut state);

    let paragraph = Paragraph::new(Text::from(display_data.content.body))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(display_data.content.title)
                .borders(Borders::ALL),
        );
    f.render_widget(paragraph, content_chunk[1]);

    let help = Paragraph::new(Text::from(display_data.help.body)).block(
        Block::default()
            .title(display_data.help.title)
            .borders(Borders::ALL),
    );
    f.render_widget(help, chunks[1]);

    match display_data.mode {
        Mode::Insert => {
            let input = Paragraph::new(display_data.new_file_name_modal.body)
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(display_data.new_file_name_modal.title)
                        .title_alignment(Alignment::Center),
                );
            let area = centered_rect(50, 3, f.size());
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(input, area);
            f.set_cursor(
                area.x + display_data.new_file_name_modal.body.width_cjk() as u16 + 1,
                area.y + 1,
            )
        }
        Mode::DeleteConfirm => {
            let input = Paragraph::new(display_data.delete_modal.body)
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(display_data.delete_modal.title)
                        .title_alignment(Alignment::Center),
                );
            let area = centered_rect(50, 3, f.size());
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(input, area);
        }
        _ => {}
    }
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(height),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
