use crate::kakisute_file::KakisuteFile;

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
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
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
}

impl Default for Tui {
    fn default() -> Self {
        Tui {
            selected_list_index: Some(0),
            items: vec![],
            mode: Mode::Normal,
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
        }
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
                    Mode::Insert => {
                        if let (KeyCode::Esc, KeyModifiers::NONE) = (code, modifiers) {
                            tui.enter_normal_mode();
                        }
                    }
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
                        (KeyCode::Char('i'), KeyModifiers::NONE) => {
                            tui.enter_insert_mode();
                        }
                        (KeyCode::Char('j'), KeyModifiers::NONE) => {
                            tui.select_next();
                        }
                        (KeyCode::Char('k'), KeyModifiers::NONE) => {
                            tui.select_previous();
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
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        let items2 = tui
            .items
            .iter()
            .map(|file| ListItem::new(file.file_name()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items2)
            .block(Block::default().title("List").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        let mut state = ListState::default();

        state.select(tui.selected_list_index);
        f.render_stateful_widget(list, chunks[1], &mut state);
    }
}
