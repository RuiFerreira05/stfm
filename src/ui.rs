use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Style,
    widgets::Block,
};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let main_box = Block::bordered().title_bottom(" q: quit ");
    frame.render_widget(&main_box, frame.area());
    let h_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [left, right] = h_layout.areas(main_box.inner(frame.area()));
    let inner_left_box = Block::bordered().border_style(Style::new().red());
    frame.render_widget(inner_left_box, left);
    let inner_right_box = Block::bordered().border_style(Style::new().blue());
    frame.render_widget(inner_right_box, right);
}
