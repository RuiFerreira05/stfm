use ratatui::{Frame, layout::{Constraint, Rows}, widgets::{ListItem, Row, Table}};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let logs = &app.logger.logs;

    let rows: Vec<Row> = logs.into_iter().map(|log| style_row(log)).collect();

    let widths = [Constraint::Length(40), Constraint::Fill(1)];

    let table = Table::new(rows, widths)
}

fn style_row(log: &crate::logger::LogMessage) -> Row {
    let log_msg = log.message;
    let log_level = log.log_level.to_string();
    let log_timestamp = log.timestamp; //TODO: format timestamp (probs different crate)
    let row: Row = Row::new(vec![log_level, log_msg]);
}
