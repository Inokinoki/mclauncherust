use crate::tuiapp::app::TUIAppState;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, BorderType, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, s: TUIAppState) {
    let size = f.size();
    let block = Block::default()
        .title("MC Launcherust - [S]tart [D]ownload [P]rint [R]efresh [Q]uit")
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    
    /* Version chunks */
    let version_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
    let installed_version_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Installed", Style::default().fg(Color::Green)));
    let all_version_block = Block::default().title("All")
        .borders(Borders::ALL);

    f.render_widget(installed_version_block, version_chunks[0]);
    f.render_widget(all_version_block, version_chunks[1]);

    /* Status chunks */
    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(chunks[1]);
    let block = Block::default()
        .title(Span::styled("Status", Style::default().fg(Color::Yellow)))
        .borders(Borders::ALL);
    f.render_widget(block, status_chunks[0]);
}
