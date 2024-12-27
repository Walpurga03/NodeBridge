use super::common::*;
use super::{components, help, tabs};
use std::time::Duration;
use crate::rpc::{MempoolInfo, NodeStatus};
use crate::ui::{StatusMessage, MessageLevel};
use ratatui::prelude::Alignment;
use ratatui::widgets::{Paragraph, Table};

enum ContentWidget<'a> {
    Text(Paragraph<'a>),
    Table(Table<'a>),
}

pub fn draw_ui(
    f: &mut Frame,
    tab: &Tab,
    show_help: bool,
    version: u64,
    height: u64,
    block_hash: &str,
    timestamp: i64,
    connections: u64,
    verification_progress: f64,
    mempool_size: u64,
    network: &str,
    update_interval: Duration,
    is_updating: bool,
    spinner_state: usize,
    mempool_info: &MempoolInfo,
    status_messages: &[StatusMessage],
    node_info: &NodeStatus,
) {
    if !show_help {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(f.size());

        let header = components::create_header(version);
        let tabs = components::create_tabs(tab);
        
        let content = match tab {
            Tab::Overview => ContentWidget::Text(tabs::create_overview(
                height,
                block_hash.to_string(),
                timestamp,
                connections,
                verification_progress,
                mempool_size,
                network.to_string(),
            )),
            Tab::BlockDetails => ContentWidget::Text(tabs::create_block_details(
                height,
                block_hash.to_string(),
                timestamp,
            )),
            Tab::Mempool => ContentWidget::Text(tabs::create_mempool(mempool_info)),
            Tab::Network => ContentWidget::Text(tabs::create_network(
                connections,
                network.to_string(),
                verification_progress,
                &node_info.peers,
            )),
            Tab::PeerList => ContentWidget::Table(tabs::create_peer_list(&node_info.peers)),
            Tab::Mining => ContentWidget::Text(tabs::create_mining()),
            Tab::Security => ContentWidget::Text(tabs::create_security()),
            Tab::Explorer => ContentWidget::Text(tabs::create_explorer()),
        };
        let footer = components::create_footer(update_interval, is_updating, spinner_state);

        f.render_widget(header, chunks[0]);
        f.render_widget(tabs, chunks[1]);
        match content {
            ContentWidget::Text(widget) => f.render_widget(widget, chunks[2]),
            ContentWidget::Table(widget) => f.render_widget(widget, chunks[2]),
        }
        f.render_widget(footer, chunks[3]);

        // Status-Nachrichten anzeigen
        if !status_messages.is_empty() {
            let message = &status_messages[0];
            let style = match message.level {
                MessageLevel::Info => Style::default().fg(Color::Blue),
                MessageLevel::Warning => Style::default().fg(Color::Yellow),
                MessageLevel::Error => Style::default().fg(Color::Red),
            };
            
            let status = Paragraph::new(message.text.clone())
                .style(style)
                .alignment(Alignment::Center);
                
            f.render_widget(status, chunks[4]);
        }
    } else {
        // Bildschirm komplett schwarz färben
        f.render_widget(Clear, f.size());
        
        // Schwarzes Hintergrund-Widget für den Hilfe-Bereich
        let area = centered_rect(80, 90, f.size());
        f.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            area
        );
        
        // Hilfe-Widget darüber rendern
        let help = help::create_help(tab);
        f.render_widget(help, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
} 