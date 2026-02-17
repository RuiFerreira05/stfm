use std::fs::DirEntry;

use ratatui::{
    Frame,
    layout::Constraint,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Row, Table},
};

use crate::{
    app::App,
    styles::{self},
    utils,
};

// pub fn render(app: &App, frame: &mut Frame) {
//     let main_box = Block::bordered().title_bottom(" q: quit ");
//     frame.render_widget(&main_box, frame.area());
//     let h_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
//     let [left, right] = h_layout.areas(main_box.inner(frame.area()));
//     let inner_left_box = Block::bordered().border_style(Style::new().red());
//     frame.render_widget(inner_left_box, left);
//     let inner_right_box = Block::bordered().border_style(Style::new().blue());
//     frame.render_widget(inner_right_box, right);
// }

pub fn render(app: &mut App, frame: &mut Frame) {
    // render based on screen
    match app.current_screen {
        crate::app::Screens::MainScreen => {
            let rows: Vec<Row> = app
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

            frame.render_stateful_widget(main_table, frame.area(), &mut app.dir_table_state);
        }
    }
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
