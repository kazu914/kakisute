use crate::ui::display_data::DisplayData;
use tui::{
    layout::{Alignment, Rect},
    widgets::Clear,
};
use unicode_width::UnicodeWidthStr;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::app_interactor::Mode;

pub const SEARCH_BOX_LENGTH: u16 = 3;
pub const HELP_BOX_LENGTH: u16 = 3;
pub const CONTENT_CHUNK_MIN_SIZE: u16 = 3;
pub const LIST_WIDTH_PERCENT: u16 = 20;
pub const CONTENT_WIDTH_PERCENT: u16 = 80;
pub const MARGIN: u16 = 1;

fn build_main_layout<B: Backend>(f: &mut Frame<B>, mode: &Mode) -> Vec<Rect> {
    match mode {
        Mode::Search => Layout::default()
            .direction(Direction::Vertical)
            .margin(MARGIN)
            .constraints(
                [
                    Constraint::Min(CONTENT_CHUNK_MIN_SIZE),
                    Constraint::Length(SEARCH_BOX_LENGTH),
                    Constraint::Length(HELP_BOX_LENGTH),
                ]
                .as_ref(),
            )
            .split(f.size()),
        _ => Layout::default()
            .direction(Direction::Vertical)
            .margin(MARGIN)
            .constraints(
                [
                    Constraint::Min(CONTENT_CHUNK_MIN_SIZE),
                    Constraint::Length(HELP_BOX_LENGTH),
                ]
                .as_ref(),
            )
            .split(f.size()),
    }
}

pub fn render<B: Backend>(f: &mut Frame<B>, display_data: DisplayData) {
    let chunks = build_main_layout(f, &display_data.mode);
    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(LIST_WIDTH_PERCENT),
                Constraint::Percentage(CONTENT_WIDTH_PERCENT),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let file_names = display_data
        .kakisute_list
        .body
        .iter()
        .map(|file_name| ListItem::new(file_name.to_string()))
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
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::Black))
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
    f.render_widget(help, chunks[chunks.len() - 1]);

    match display_data.mode {
        Mode::Insert => {
            let input = Paragraph::new(display_data.new_filename.body.clone())
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(display_data.new_filename.title)
                        .title_alignment(Alignment::Center),
                );
            let area = centered_rect(50, 3, f.size());
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(input, area);
            f.set_cursor(
                area.x + display_data.new_filename.body.width_cjk() as u16 + 1,
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
        Mode::Search => {
            let input = Paragraph::new(display_data.search_query.body.clone())
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(display_data.search_query.title),
                );
            f.render_widget(input, chunks[1]);
            f.set_cursor(
                chunks[1].x + display_data.search_query.body.width_cjk() as u16 + 1,
                chunks[1].y + 1,
            )
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
