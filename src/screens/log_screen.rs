use ratatui::{
    layout::Constraint,
    text::Text,
    widgets::{canvas::Line, Row, Table},
    Frame,
};

use crate::{
    app::App,
    logger::{LogLevel, LogMessage},
    styles,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let logs = &app.logger.logs;

    let rows: Vec<Row> = logs.into_iter().map(|log| style_row(log)).collect();

    let widths = [
        Constraint::Length(6),
        Constraint::Fill(1),
        Constraint::Length(20),
    ];

    let table = Table::new(rows, widths);

    app.ui.log_list_state.select_last();
    frame.render_stateful_widget(table, frame.area(), &mut app.ui.log_list_state);
}

fn style_row(log: &LogMessage) -> Row<'_> {
    let log_level = Text::from(log.log_level.to_string()).style(match log.log_level {
        LogLevel::Fatal => styles::mocha::RED,
        LogLevel::Error => styles::mocha::RED,
        _ => styles::mocha::TEXT,
    });
    let log_msg = Text::from(log.message.to_string());
    let log_timestamp = Text::from(log.timestamp.format("%d/%m/%Y %H:%M:%S").to_string());
    let row = Row::new(vec![log_level, log_msg, log_timestamp]);

    return row;
}
