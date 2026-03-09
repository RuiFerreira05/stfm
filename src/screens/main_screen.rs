use std::fs::DirEntry;

use ratatui::{
    Frame,
    layout::Constraint,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Row, Table},
};

use crate::{app::App, styles, utils};

pub fn render(app: &mut App, frame: &mut Frame) {
    let rows: Vec<Row> = app
        .dir
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
                .dir
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
                .title(title),
        )
        .style(Style::new().bg(styles::mocha::BASE).fg(styles::mocha::TEXT))
        .highlight_symbol(" -> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
        .row_highlight_style(Style::new().italic().bold());

    frame.render_stateful_widget(main_table, frame.area(), &mut app.ui.dir_table_state);
}

fn style_dir(i: usize, dir: &DirEntry) -> Row<'_> {
    let name = dir.file_name().into_string().unwrap();
    let size_fmt = utils::format_size(dir);

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
