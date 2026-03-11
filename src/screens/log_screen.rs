use ratatui::{
    Frame,
    layout::Constraint,
    text::Text,
    widgets::{Row, Table},
};

use crate::{
    app::App,
    logger::{LogLevel, LogMessage},
    styles::{self, modules::main_block},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let logs = &app.logger.logs;

    let rows: Vec<Row> = logs.into_iter().map(|log| style_row(log)).collect();

    let widths = [
        Constraint::Length(7),
        Constraint::Fill(1),
        Constraint::Length(22),
    ];

    let table = Table::new(rows, widths).block(main_block("Logs".to_string(), "".to_string()));

    app.ui.log_list_state.select_last();
    frame.render_stateful_widget(table, frame.area(), &mut app.ui.log_list_state);
}

fn style_row(log: &LogMessage) -> Row<'_> {
    let log_level = Text::from(" ".to_string() + log.log_level.to_string().as_str()).style(
        match log.log_level {
            LogLevel::Fatal => styles::mocha::RED,
            LogLevel::Error => styles::mocha::RED,
            _ => styles::mocha::TEXT,
        },
    );
    let log_msg = Text::from("│ ".to_string() + log.message.to_string().as_str());
    let log_timestamp = Text::from(
        "│ ".to_string()
            + log
                .timestamp
                .format("%d/%m/%Y %H:%M:%S")
                .to_string()
                .as_str(),
    );
    let row = Row::new(vec![log_level, log_msg, log_timestamp]);

    return row;
}
