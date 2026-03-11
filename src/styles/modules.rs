use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::Block,
};

use crate::styles;

pub fn main_block(title: String, err_msg: String) -> Block<'static> {
    let title_line = Line::from(title).italic().fg(styles::mocha::SUBTEXT_0);
    Block::bordered()
        .border_style(Style::new().fg(styles::mocha::LAVENDER))
        .title(title_line)
        .title_bottom(Line::from(err_msg).style(styles::mocha::RED))
}
