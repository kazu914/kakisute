use crate::ui::display_data::DisplayData;
use tui::{
    layout::{Alignment, Rect},
    widgets::Clear,
};
use unicode_width::UnicodeWidthStr;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::app_interactor::Mode;

pub fn render<B: Backend>(f: &mut Frame<B>, display_data: DisplayData) {
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
