use crate::tuiapp::app::TUIAppState;
use crate::tuiapp::app::StatefulList;

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

use crate::tuiapp::{ Focus };

pub fn draw<B: Backend>(f: &mut Frame<B>, s: &mut TUIAppState) {
    match s.focused {
        Focus::DOWNLOAD_PAGE => {
            drawDonwloadPage(f, s);
        }
        _ => {
            drawInfoPage(f, s);
        }
    }
}

fn drawDonwloadPage<B: Backend>(f: &mut Frame<B>, s: &mut TUIAppState) {
    let size = f.size();
    let block = Block::default()
        .title("MC Launcherust - [[version placeholder]] [R]efresh [C]ancel")
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);
}

fn drawInfoPage<B: Backend>(f: &mut Frame<B>, s: &mut TUIAppState) {
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
    match &mut s.installed_items {
        Some(list) => {
            // Iterate through all elements in the `items` app and append some debug text to it.
            let items: Vec<ListItem> = list
                .items   
                .iter()
                .map(|i| {
                    ListItem::new(Spans::from(String::from(i.id.clone())))
                })
                .collect();
            
            match s.focused {
                Focus::INSTALLED_VERSION_LIST => {
                    let installed_version_block = Block::default()
                        .borders(Borders::ALL)
                        .title(Span::styled("Installed", Style::default().fg(Color::Green)));
                    // Create a List from installed list items and highlight the currently selected one
                    let items = List::new(items)
                        .block(installed_version_block)
                        .highlight_style(
                            Style::default()
                                .bg(Color::LightGreen)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");
                    // We can now render the item list
                    f.render_stateful_widget(items, version_chunks[0], &mut list.state);
                }
                _ => {
                    let installed_version_block = Block::default()
                        .borders(Borders::ALL)
                        .title("Installed");
                    // Create a List from installed list items and highlight the currently selected one
                    let items = List::new(items)
                        .block(installed_version_block)
                        .highlight_style(
                            Style::default()
                                .bg(Color::LightGreen)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");
                    // We can now render the item list
                    f.render_stateful_widget(items, version_chunks[0], &mut list.state);
                }
            }
        }
        None => {
            /* Failed to load available versions */
            s.focused = Focus::ALL_VERSION_LIST;    // Force to focus here
            let all_version_block = Block::default()
                .title(Span::styled("All", Style::default().fg(Color::Red)))
                .borders(Borders::ALL);
            let text = vec![
                Spans::from(
                    Span::styled("No available version, please check your network",
                        Style::default().fg(Color::Red))
                ),
                Spans::from("press [R] to refresh"),
            ];
            let version_not_loaded_paragraph = Paragraph::new(text.clone())
                .block(all_version_block)
                .alignment(Alignment::Center);
            
            f.render_widget(version_not_loaded_paragraph, version_chunks[1]);
        }
    }


    match &mut s.stateful_items {
        Some(list) => {
            // Iterate through all elements in the `items` app and append some debug text to it.
            let items: Vec<ListItem> = list
                .items   
                .iter()
                .map(|i| {
                    ListItem::new(Spans::from(String::from(i.id.clone())))
                })
                .collect();
            
            match s.focused {
                Focus::ALL_VERSION_LIST => {
                    let all_version_block = Block::default()
                        .title(Span::styled("All", Style::default().fg(Color::Green)))
                        .borders(Borders::ALL);
                    // Create a List from all list items and highlight the currently selected one
                    let items = List::new(items)
                        .block(all_version_block)
                        .highlight_style(
                            Style::default()
                                .bg(Color::LightGreen)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");
                    // We can now render the item list
                    f.render_stateful_widget(items, version_chunks[1], &mut list.state);
                }
                _ => {
                    let all_version_block = Block::default()
                        .title("All")
                        .borders(Borders::ALL);
                    // Create a List from all list items and highlight the currently selected one
                    let items = List::new(items)
                        .block(all_version_block)
                        .highlight_style(
                            Style::default()
                                .bg(Color::LightGreen)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");
                    // We can now render the item list
                    f.render_stateful_widget(items, version_chunks[1], &mut list.state);
                }
            }
        }
        None => {
            /* Failed to load available versions */
            s.focused = Focus::ALL_VERSION_LIST;    // Force to focus here
            let all_version_block = Block::default()
                .title(Span::styled("All", Style::default().fg(Color::Red)))
                .borders(Borders::ALL);
            let text = vec![
                Spans::from(
                    Span::styled("No available version, please check your network",
                        Style::default().fg(Color::Red))
                ),
                Spans::from("press [R] to refresh"),
            ];
            let version_not_loaded_paragraph = Paragraph::new(text.clone())
                .block(all_version_block)
                .alignment(Alignment::Center);
            
            f.render_widget(version_not_loaded_paragraph, version_chunks[1]);
        }
    }

    /* Status chunks */
    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(chunks[1]);
    match s.focused {
        Focus::STATUS_LIST => {
            let block = Block::default()
                .title(Span::styled("Status", Style::default().fg(Color::Yellow)))
                .borders(Borders::ALL);
            f.render_widget(block, status_chunks[0]);
        }
        Focus::ALL_VERSION_LIST => {
            // Display choosed info in all version list
            match &s.stateful_items {
                Some(list) => {
                    match list.state.selected() {
                        Some(i) => {
                            let item = list.items.get(i).unwrap();

                            let mut is_installed = false;
                            match &s.installed_items {
                                Some(items) => {
                                    let mut iter = items.items.iter();
                                    is_installed = iter.any(|x| x.id == item.id);
                                }
                                None => {}
                            }

                            let text = vec![
                                Spans::from(format!("ID: {}", item.id)),
                                Spans::from(format!("Type: {}", item.r#type)),
                                Spans::from(format!("URL: {}", item.url)),
                                Spans::from(format!("Release Time: {}", item.releaseTime)),
                                Spans::from(format!("Installed: {}", if is_installed { "Yes" } else { "No" })),
                            ];
                            let block = Block::default()
                                .title("Status")
                                .borders(Borders::ALL);
                            let paragraph = Paragraph::new(text.clone())
                                .block(block)
                                .alignment(Alignment::Left);
                            f.render_widget(paragraph, status_chunks[0]);
                        }
                        None => {
                            let block = Block::default()
                                .title("Status")
                                .borders(Borders::ALL);
                            f.render_widget(block, status_chunks[0]);
                        }
                    }
                }
                None => {
                    let block = Block::default()
                        .title("Status")
                        .borders(Borders::ALL);
                    f.render_widget(block, status_chunks[0]);
                }
            }
        }
        _ => {
            let block = Block::default()
                .title("Status")
                .borders(Borders::ALL);
            f.render_widget(block, status_chunks[0]);
        }
    }
}
