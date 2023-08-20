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

use super::{app_interactor::Mode, display_data::BlockData};

pub const SEARCH_BOX_LENGTH: u16 = 3;
pub const HELP_BOX_LENGTH: u16 = 3;
pub const CONTENT_CHUNK_MIN_SIZE: u16 = 3;
pub const LIST_WIDTH_PERCENT: u16 = 20;
pub const CONTENT_WIDTH_PERCENT: u16 = 80;
pub const MARGIN: u16 = 1;

/// Build main layout
/// If need_search_box is true, build layout with search box
fn build_main_layout<B: Backend>(f: &mut Frame<B>, need_search_box: bool) -> Vec<Rect> {
    if need_search_box {
        Layout::default()
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
            .split(f.size())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(MARGIN)
            .constraints(
                [
                    Constraint::Min(CONTENT_CHUNK_MIN_SIZE),
                    Constraint::Length(HELP_BOX_LENGTH),
                ]
                .as_ref(),
            )
            .split(f.size())
    }
}

fn generate_filename_list<'a>(kakisute_list: BlockData<Vec<&'a str>>, mode: &Mode) -> List<'a> {
    let file_names = kakisute_list
        .body
        .iter()
        .map(|file_name| ListItem::new(file_name.to_string()))
        .collect::<Vec<ListItem>>();

    List::new(file_names)
        .block(
            Block::default()
                .title(kakisute_list.title.clone())
                .borders(Borders::ALL)
                .border_style(match mode {
                    Mode::Normal => Style::default().fg(Color::Blue),
                    _ => Style::default(),
                }),
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::Black))
        .highlight_symbol(">>")
}

fn generate_kakisute_content<'a>(content: BlockData<String>) -> Paragraph<'a> {
    Paragraph::new(Text::from(content.body))
        .wrap(Wrap { trim: false })
        .block(Block::default().title(content.title).borders(Borders::ALL))
}

fn generate_help<'a>(help: BlockData<String>) -> Paragraph<'a> {
    Paragraph::new(Text::from(help.body))
        .block(Block::default().title(help.title).borders(Borders::ALL))
}

fn generate_input_box<'a>(new_filename: &BlockData<String>) -> Paragraph<'a> {
    Paragraph::new(new_filename.body.clone())
        .style(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(new_filename.title.clone())
                .title_alignment(Alignment::Center),
        )
}

fn generate_search_box<'a>(search_query: &BlockData<String>, mode: &Mode) -> Paragraph<'a> {
    // Use blue color only if focused
    let border_color = if mode == &Mode::Search {
        Color::Blue
    } else {
        Color::White
    };
    Paragraph::new(search_query.body.clone())
        .style(Style::default().fg(border_color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(search_query.title.clone()),
        )
}

fn generate_delete_modal<'a>(delete_modal: &BlockData<&'a str>) -> Paragraph<'a> {
    Paragraph::new(delete_modal.body)
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(delete_modal.title.clone())
                .title_alignment(Alignment::Center),
        )
}

pub fn render<B: Backend>(f: &mut Frame<B>, display_data: DisplayData) {
    let chunks = build_main_layout(f, display_data.need_search_box);
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

    let filename_list = generate_filename_list(display_data.kakisute_list, &display_data.mode);
    let mut state = ListState::default();
    state.select(display_data.index);
    f.render_stateful_widget(filename_list, content_chunk[0], &mut state);

    let kakisute_content = generate_kakisute_content(display_data.content);
    f.render_widget(kakisute_content, content_chunk[1]);

    let help = generate_help(display_data.help);
    f.render_widget(help, chunks[chunks.len() - 1]);

    if display_data.need_search_box {
        let search_box = generate_search_box(&display_data.search_query, &display_data.mode);
        f.render_widget(search_box, chunks[1]);

        // Show the cursor when in search mode
        if display_data.mode == Mode::Search {
            f.set_cursor(
                chunks[1].x + display_data.search_query.body.width_cjk() as u16 + 1,
                chunks[1].y + 1,
            )
        }
    }

    match display_data.mode {
        Mode::Insert => {
            let input = generate_input_box(&display_data.new_filename);
            let area = centered_rect(50, 3, f.size());
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(input, area);
            f.set_cursor(
                area.x + display_data.new_filename.body.width_cjk() as u16 + 1,
                area.y + 1,
            )
        }
        Mode::DeleteConfirm => {
            let input = generate_delete_modal(&display_data.delete_modal);
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
