use ratatui::{
    Frame,
    layout::Constraint,
    widgets::{Row, Table},
};

use crate::{app::App, logger::LogMessage};

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
    let log_msg = log.message.to_string();
    let log_level = log.log_level.to_string();
    let log_timestamp = log.timestamp.format("%d/%m/%Y %H:%M:%S").to_string();
    let row = Row::new(vec![log_level, log_msg, log_timestamp]);

    return row;
}
