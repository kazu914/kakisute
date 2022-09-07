use crate::{app::App, kakisute_file::KakisuteFile};
use tui::{
    layout::{Alignment, Rect},
    widgets::Clear,
};
use unicode_width::UnicodeWidthStr;

use std::{io, usize};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

enum Mode {
    Normal,
    Insert,
    DeleteConfirm,
}

struct Tui {
    selected_list_index: Option<usize>,
    items: Vec<KakisuteFile>,
    mode: Mode,
    new_file_name: String,
}

impl Default for Tui {
    fn default() -> Self {
        Tui {
            selected_list_index: Some(0),
            items: vec![],
            mode: Mode::Normal,
            new_file_name: String::new(),
        }
    }
}

impl Tui {
    fn new(kakisute_file_list: Vec<KakisuteFile>) -> Self {
        let index = if kakisute_file_list.is_empty() {
            None
        } else {
            Some(0)
        };
        Tui {
            selected_list_index: index,
            items: kakisute_file_list,
            mode: Mode::Normal,
            new_file_name: String::new(),
        }
    }

    fn reload(&mut self, kakisute_file_list: Vec<KakisuteFile>) {
        let index = if kakisute_file_list.is_empty() {
            None
        } else {
            Some(0)
        };

        self.selected_list_index = index;
        self.items = kakisute_file_list;
        self.mode = Mode::Normal;
    }
    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }
    fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }
    fn enter_delete_mode(&mut self) {
        self.mode = Mode::DeleteConfirm;
    }
    fn select_next(&mut self) {
        match self.selected_list_index {
            Some(n) => {
                if n != self.items.len() - 1 {
                    self.selected_list_index = Some(n + 1);
                } else {
                    self.selected_list_index = Some(0);
                }
            }
            None => {}
        }
    }
    fn select_previous(&mut self) {
        match self.selected_list_index {
            Some(n) => {
                if n > 0 {
                    self.selected_list_index = Some(n - 1);
                } else {
                    self.selected_list_index = Some(self.items.len() - 1);
                }
            }
            None => {}
        }
    }
    fn clear_file_name(&mut self) {
        self.new_file_name = String::new();
    }
}

pub fn run_app(app: &mut App) -> Result<(), io::Error> {
    let kakisute_list = app.get_kakisute_list();
    let mut tui = Tui::new(kakisute_list);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            render(app, f, &tui);
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
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        app.create_kakisute(Some(tui.new_file_name.clone()));
                        execute!(
                            terminal.backend_mut(),
                            EnterAlternateScreen,
                            EnableMouseCapture
                        )?;
                        terminal.clear()?;
                        app.reload();
                        tui.reload(app.get_kakisute_list());
                        tui.clear_file_name();
                    }
                    _ => {}
                },
                Mode::Normal => match (code, modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;

                        terminal.show_cursor()?;
                        return Ok(());
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
                        if let Some(index) = tui.selected_list_index {
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            app.edit_by_index(index);
                            execute!(
                                terminal.backend_mut(),
                                EnterAlternateScreen,
                                EnableMouseCapture
                            )?;
                            terminal.clear()?;
                        }
                    }
                    (KeyCode::Char('n'), KeyModifiers::NONE) => {
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        app.create_kakisute(None);
                        execute!(
                            terminal.backend_mut(),
                            EnterAlternateScreen,
                            EnableMouseCapture
                        )?;
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
                    (KeyCode::Esc, KeyModifiers::NONE)
                    | (KeyCode::Char('n'), KeyModifiers::NONE) => {
                        tui.enter_normal_mode();
                    }
                    (KeyCode::Char('Y'), KeyModifiers::SHIFT) => {
                        if let Some(index) = tui.selected_list_index {
                            app.delete_by_index(index);
                            app.reload();
                            tui.reload(app.get_kakisute_list());
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}

fn render<B: Backend>(app: &mut App, f: &mut Frame<B>, tui: &Tui) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(f.size());

    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[0]);

    let file_names = tui
        .items
        .iter()
        .map(|file| ListItem::new(file.file_name()))
        .collect::<Vec<ListItem>>();

    let list = List::new(file_names)
        .block(
            Block::default()
                .title("List")
                .borders(Borders::ALL)
                .border_style(match tui.mode {
                    Mode::Normal => Style::default().fg(Color::Blue),
                    _ => Style::default(),
                }),
        )
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    let mut state = ListState::default();
    state.select(tui.selected_list_index);

    f.render_stateful_widget(list, content_chunk[0], &mut state);

    let content = match tui.selected_list_index {
        Some(index) => app.get_kakisute_contetent(index),
        None => None,
    };

    let paragraph = Paragraph::new(match content {
        Some(content) => Text::from(content),
        None => Text::from("<No file is selected>"),
    })
    .wrap(Wrap { trim: false })
    .block(Block::default().title("Content").borders(Borders::ALL));
    f.render_widget(paragraph, content_chunk[1]);

    let help = Paragraph::new(Text::from(match tui.mode {
            Mode::Normal => {
                "esc: Quit, j: Down, k: Up, e: Edit, n: Create new, N: Create new with file name, d: Delete"
            }

            Mode::Insert => "esc: Enter normal mode, Enter: Open editor",
            Mode::DeleteConfirm => "esc/n: Cancel, Y: delete",
        }))
        .block(Block::default().title("Help").borders(Borders::ALL));
    f.render_widget(help, chunks[1]);

    match tui.mode {
        Mode::Insert => {
            let input = Paragraph::new(tui.new_file_name.as_ref())
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Input new file name")
                        .title_alignment(Alignment::Center),
                );
            let area = centered_rect(50, 3, f.size());
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(input, area);
            f.set_cursor(
                area.x + tui.new_file_name.width_cjk() as u16 + 1,
                area.y + 1,
            )
        }
        Mode::DeleteConfirm => {
            let input = Paragraph::new("Are you sure you want to delete? (Y/n)")
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Confirm Modal")
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
