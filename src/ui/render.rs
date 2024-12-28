use super::common::*;
use super::{components, help};
use std::time::Duration;
use crate::rpc::{BitcoinRPC, MempoolInfo, NodeStatus};
use crate::ui::{StatusMessage, MessageLevel};
use crate::ui::tabs::block_details::BlockSearchMode;
use ratatui::prelude::Alignment;
use ratatui::widgets::{Paragraph, Table};
use super::tabs::{
    render_node_info, render_block_details, render_mempool,
    render_network, render_peer_list, render_mining,
    render_security, render_explorer
};

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
    rpc_client: &Option<BitcoinRPC>,
    block_search_mode: &BlockSearchMode,
    block_input_active: bool,
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
            Tab::Dashboard => ContentWidget::Text(render_node_info(
                &network,
                connections,
                height,
                height,
                node_info.difficulty,
                node_info.chain_work.clone(),
                verification_progress,
                node_info.initial_block_download,
                node_info.size_on_disk,
                node_info.pruned,
                mempool_size,
                "",
            )),
            Tab::BlockDetails => {
                match rpc_client {
                    Some(client) => {
                        let block_height = match block_search_mode {
                            BlockSearchMode::Latest => node_info.height.to_string(),
                            BlockSearchMode::Custom(ref input) => input.clone(),
                        };
                        
                        match client.get_block_details(&block_height) {
                            Ok(block_details) => ContentWidget::Text(render_block_details(
                                &block_details,
                                block_search_mode,
                                block_input_active,
                            )),
                            Err(_) => ContentWidget::Text(Paragraph::new("Block konnte nicht gefunden werden")
                                .style(Style::default().fg(Color::Red)))
                        }
                    },
                    None => ContentWidget::Text(Paragraph::new("Keine Verbindung zum Bitcoin Node")
                        .style(Style::default().fg(Color::Red)))
                }
            },
            Tab::Mempool => ContentWidget::Text(render_mempool(mempool_info)),
            Tab::Network => ContentWidget::Text(render_network(
                connections,
                network.to_string(),
                verification_progress,
                &node_info.peers,
            )),
            Tab::PeerList => ContentWidget::Table(render_peer_list(&node_info.peers)),
            Tab::Mining => ContentWidget::Text(render_mining()),
            Tab::Security => ContentWidget::Text(render_security()),
            Tab::Explorer => ContentWidget::Text(render_explorer()),
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
        // Erst alles löschen
        f.render_widget(Clear, f.size());
        
        // Dann den gesamten Bildschirm schwarz färben
        f.render_widget(
            Block::default()
                .style(Style::default().bg(Color::Black))
                .borders(Borders::NONE),
            f.size()
        );
        
        // Hilfe-Fenster-Bereich definieren
        let area = centered_rect(80, 90, f.size());
        
        // Nochmal explizit den Hilfe-Bereich schwarz färben
        f.render_widget(
            Block::default()
                .style(Style::default().bg(Color::Black))
                .borders(Borders::NONE),
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