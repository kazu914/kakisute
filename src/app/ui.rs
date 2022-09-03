use crate::{kakisute_file::KakisuteFile, operation};
use tui::{
    layout::{Alignment, Rect},
    widgets::Clear,
};
use unicode_width::UnicodeWidthStr;

use super::App;

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
}

struct Tui {
    selected_list_index: Option<usize>,
    items: Vec<KakisuteFile>,
    mode: Mode,
    new_filename: String,
}

impl Default for Tui {
    fn default() -> Self {
        Tui {
            selected_list_index: Some(0),
            items: vec![],
            mode: Mode::Normal,
            new_filename: String::new(),
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
            new_filename: String::new(),
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
    fn clear_filename(&mut self) {
        self.new_filename = String::new();
    }
}

impl App {
    pub fn ui(&mut self) -> Result<(), io::Error> {
        let kakisute_list = self.kakisute_list.get_list();
        let mut tui = Tui::new(kakisute_list);
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                self.render(f, &tui);
            })?;

            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match tui.mode {
                    Mode::Insert => match (code, modifiers) {
                        (KeyCode::Esc, KeyModifiers::NONE) => {
                            tui.clear_filename();
                            tui.enter_normal_mode();
                        }
                        (KeyCode::Char(c), KeyModifiers::NONE) => {
                            tui.new_filename.push(c);
                        }
                        (KeyCode::Backspace, KeyModifiers::NONE) => {
                            tui.new_filename.pop();
                        }
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            self.create_kakisute(Some(tui.new_filename.clone()));
                            execute!(
                                terminal.backend_mut(),
                                EnterAlternateScreen,
                                EnableMouseCapture
                            )?;
                            terminal.clear()?;
                            self.reload();
                            tui.reload(self.kakisute_list.get_list());
                            tui.clear_filename();
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
                            let kakisute = self.get_kakisute(tui.selected_list_index);
                            match kakisute {
                                Some(kakisute) => {
                                    execute!(
                                        terminal.backend_mut(),
                                        LeaveAlternateScreen,
                                        DisableMouseCapture
                                    )?;
                                    operation::edit(&self.data_dir, kakisute.file_name());
                                    execute!(
                                        terminal.backend_mut(),
                                        EnterAlternateScreen,
                                        EnableMouseCapture
                                    )?;
                                    terminal.clear()?;
                                }
                                None => {}
                            }
                        }
                        (KeyCode::Char('n'), KeyModifiers::NONE) => {
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            self.create_kakisute(None);
                            execute!(
                                terminal.backend_mut(),
                                EnterAlternateScreen,
                                EnableMouseCapture
                            )?;
                            terminal.clear()?;
                            self.reload();
                            tui.reload(self.kakisute_list.get_list());
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>, tui: &Tui) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
            .split(f.size());

        let content_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[0]);

        let items2 = tui
            .items
            .iter()
            .map(|file| ListItem::new(file.file_name()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items2)
            .block(
                Block::default()
                    .title("List")
                    .borders(Borders::ALL)
                    .border_style(match tui.mode {
                        Mode::Normal => Style::default().fg(Color::Blue),
                        Mode::Insert => Style::default(),
                    }),
            )
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        let mut state = ListState::default();
        state.select(tui.selected_list_index);

        f.render_stateful_widget(list, content_chunk[0], &mut state);

        let content = self.get_kakisute_contetent(tui.selected_list_index);
        if let Some(content) = content {
            let paragraph = Paragraph::new(Text::from(content))
                .wrap(Wrap { trim: false })
                .block(Block::default().title("Content").borders(Borders::ALL));
            f.render_widget(paragraph, content_chunk[1])
        }

        let help = Paragraph::new(Text::from(match tui.mode {
            Mode::Normal => {
                "esc: Quit, j: Down, k: Up, e: Edit, n: Create new, N: Create new with file name"
            }

            Mode::Insert => "esc: Enter normal mode, Enter: Open editor",
        }))
        .block(Block::default().title("Help").borders(Borders::ALL));
        f.render_widget(help, chunks[1]);

        match tui.mode {
            Mode::Normal => {}
            Mode::Insert => {
                let input = Paragraph::new(tui.new_filename.as_ref())
                    .style(match tui.mode {
                        Mode::Normal => Style::default(),
                        Mode::Insert => Style::default().fg(Color::Blue).bg(Color::Black),
                    })
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Input new file name")
                            .title_alignment(Alignment::Center),
                    );
                let area = centered_rect(50, 3, f.size());
                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(input, area);
                f.set_cursor(area.x + tui.new_filename.width_cjk() as u16 + 1, area.y + 1)
            }
        }
    }

    fn get_kakisute_contetent(&self, index: Option<usize>) -> Option<String> {
        let kakisute = self.get_kakisute(index);
        match kakisute {
            Some(kakisute) => operation::get_content(&self.data_dir, kakisute.file_name()),
            None => None,
        }
    }

    fn get_kakisute(&self, index: Option<usize>) -> Option<&KakisuteFile> {
        self.kakisute_list.get(index)
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
