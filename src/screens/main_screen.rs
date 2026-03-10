use std::fs::DirEntry;

use ratatui::{
    Frame,
    layout::Constraint,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Row, Table},
};

use crate::{app::App, styles};

pub fn render(app: &mut App, frame: &mut Frame) {
    let rows: Vec<Row> = app
        .navigator
        .dir_items
        .iter()
        .enumerate()
        .map(|(i, dir)| style_dir(i, dir))
        .collect();

    let widths = [Constraint::Fill(1), Constraint::Length(40)];

    // TABLE STYLING
    let title_str = String::from(
        " ".to_string()
            + app
                .navigator
                .root_dir
                .to_str()
                .unwrap_or("")
                .replace("\\", "/")
                .as_str(),
    ) + "/ ";
    let title = Line::from(title_str).italic().fg(styles::mocha::SUBTEXT_0);
    let main_table = Table::new(rows, widths)
        .block(
            Block::bordered()
                .border_style(Style::new().fg(styles::mocha::LAVENDER))
                .title(title)
                .title_bottom(Line::from(app.ui.error_message.as_str()).style(styles::mocha::RED)),
        )
        .style(Style::new().bg(styles::mocha::BASE).fg(styles::mocha::TEXT))
        .highlight_symbol(" -> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
        .row_highlight_style(Style::new().italic().bold());

    frame.render_stateful_widget(main_table, frame.area(), &mut app.ui.dir_table_state);
}

fn style_dir(i: usize, dir: &DirEntry) -> Row<'_> {
    let name = dir.file_name().into_string().unwrap();
    let size_fmt = format_size(dir);

    let mut style = if let Ok(file_type) = dir.file_type() {
        if file_type.is_dir() {
            Style::new().fg(styles::mocha::MAUVE)
        } else {
            Style::new().fg(styles::mocha::TEXT)
        }
    } else {
        Style::new().fg(styles::mocha::TEXT)
    };

    if i % 2 == 0 {
        style = style.bg(styles::mocha::MANTLE);
    }

    Row::new(vec![name, size_fmt]).style(style)
}

pub fn format_size(dir: &DirEntry) -> String {
    if let Ok(file_type) = dir.file_type() {
        if file_type.is_dir() {
            return String::from("----");
        }
    }
    if let Ok(metadata) = dir.metadata() {
        let mut size = metadata.len();
        let units = ["B", "KB", "MB", "GB", "TB"];
        for (i, unit) in units.iter().enumerate() {
            if size < 1024 || i == units.len() - 1 {
                return size.to_string() + unit;
            }
            size /= 1024;
        }
        unreachable!()
    } else {
        String::from("----")
    }
}
