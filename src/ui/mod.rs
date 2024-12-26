use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use crate::rpc::{BitcoinRPC, NodeStatus};
use chrono::{DateTime, TimeZone, Utc};
use std::time::{Duration, Instant};

// UI-Rendering Funktionen in einem separaten Modul
mod render {
    use super::*;

    pub fn draw_ui(
        f: &mut Frame,
        tab: &Tab,
        version: u64,
        height: u64,
        block_hash: &str,
        timestamp: i64,
        connections: u64,
        verification_progress: f64,
        mempool_size: u64,
        network: &str,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Footer
            ])
            .split(f.size());

        let header = create_header(version);
        let tabs = create_tabs(tab);
        let content = match tab {
            Tab::Overview => create_overview(height, block_hash, timestamp, connections, 
                                          verification_progress, mempool_size, network),
            Tab::BlockDetails => create_block_details(height, block_hash, timestamp),
            Tab::Mempool => create_mempool(mempool_size),
            Tab::Network => create_network(connections, network, verification_progress),
        };
        let footer = create_footer();

        f.render_widget(header, chunks[0]);
        f.render_widget(tabs, chunks[1]);
        f.render_widget(content, chunks[2]);
        f.render_widget(footer, chunks[3]);
    }

    fn create_tabs(current_tab: &Tab) -> Paragraph<'static> {
        let tabs = vec![
            (Tab::Overview, "Übersicht"),
            (Tab::BlockDetails, "Block Details"),
            (Tab::Mempool, "Mempool"),
            (Tab::Network, "Netzwerk"),
        ];

        let mut spans = Vec::new();
        for (i, (tab, name)) in tabs.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(" | "));
            }
            
            let style = if tab == current_tab {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            
            spans.push(Span::styled(
                format!("[{}] {}", (i + 1), name),
                style
            ));
        }

        Paragraph::new(Line::from(spans))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(" Navigation "))
    }

    fn create_header(version: u64) -> Paragraph<'static> {
        let version_str = format!("Bitcoin Core v{}.{}.{}",
            version / 10000,
            (version % 10000) / 100,
            version % 100
        );
        Paragraph::new(version_str)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Node Info "))
    }

    fn create_overview(
        height: u64,
        block_hash: &str,
        timestamp: i64,
        connections: u64,
        verification_progress: f64,
        mempool_size: u64,
        network: &str,
    ) -> Paragraph<'static> {
        let content_text = vec![
            Line::from(vec![
                Span::styled("Netzwerk: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    network.to_uppercase(),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Blockhöhe: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    height.to_string(),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::styled("Block Hash: ", Style::default().fg(Color::Gray)),
                Span::raw(block_hash.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Zeitstempel: ", Style::default().fg(Color::Gray)),
                Span::raw(format_timestamp(timestamp)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Verbindungen: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} Peers", connections),
                    Style::default().fg(if connections > 0 { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(vec![
                Span::styled("Sync: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:.2}%", verification_progress * 100.0),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::styled("Mempool: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} Transaktionen", mempool_size),
                    Style::default().fg(Color::Blue)
                ),
            ]),
        ];

        Paragraph::new(content_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(" Node Status "))
    }

    fn create_block_details(height: u64, block_hash: &str, timestamp: i64) -> Paragraph<'static> {
        let content = vec![
            Line::from(vec![
                Span::styled("Block Höhe: ", Style::default().fg(Color::Gray)),
                Span::styled(height.to_string(), Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Hash: ", Style::default().fg(Color::Gray)),
                Span::raw(block_hash.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Zeit: ", Style::default().fg(Color::Gray)),
                Span::raw(format_timestamp(timestamp)),
            ]),
        ];

        Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(" Block Details "))
    }

    fn create_mempool(size: u64) -> Paragraph<'static> {
        let content = vec![
            Line::from(vec![
                Span::styled("Transaktionen im Mempool: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    size.to_string(),
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];

        Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(" Mempool Status "))
    }

    fn create_network(connections: u64, network: &str, sync_progress: f64) -> Paragraph<'static> {
        let content = vec![
            Line::from(vec![
                Span::styled("Netzwerk: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    network.to_uppercase(),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Verbindungen: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} Peers", connections),
                    Style::default().fg(if connections > 0 { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(vec![
                Span::styled("Synchronisation: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:.2}%", sync_progress * 100.0),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
        ];

        Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(" Netzwerk Status "))
    }

    fn create_footer() -> Paragraph<'static> {
        Paragraph::new(" [Q] Beenden | [R] Aktualisieren | Auto-Update: 5s ")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)))
    }
}

#[derive(PartialEq)]
enum Tab {
    Overview,
    BlockDetails,
    Mempool,
    Network,
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    rpc_client: BitcoinRPC,
    node_info: NodeStatus,
    last_update: Instant,
    update_interval: Duration,
    current_tab: Tab,
}

impl UI {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let rpc_client = BitcoinRPC::new()?;
        let node_info = rpc_client.test_connection()?;

        Ok(Self {
            terminal,
            rpc_client,
            node_info,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(5),
            current_tab: Tab::Overview,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.last_update.elapsed() >= self.update_interval {
                if let Ok(info) = self.rpc_client.test_connection() {
                    self.node_info = info;
                    self.last_update = Instant::now();
                }
            }

            let version = self.node_info.version;
            let height = self.node_info.height;
            let block_hash = self.node_info.block_hash.clone();
            let timestamp = self.node_info.timestamp;
            let connections = self.node_info.connections;
            let verification_progress = self.node_info.verification_progress;
            let mempool_size = self.node_info.mempool_size;
            let network = self.node_info.network.clone();

            self.terminal.draw(|f| {
                render::draw_ui(
                    f,
                    &self.current_tab,
                    version,
                    height,
                    &block_hash,
                    timestamp,
                    connections,
                    verification_progress,
                    mempool_size,
                    &network,
                )
            })?;

            if crossterm::event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => self.refresh_data()?,
                        KeyCode::Char('1') => self.current_tab = Tab::Overview,
                        KeyCode::Char('2') => self.current_tab = Tab::BlockDetails,
                        KeyCode::Char('3') => self.current_tab = Tab::Mempool,
                        KeyCode::Char('4') => self.current_tab = Tab::Network,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn refresh_data(&mut self) -> Result<()> {
        if let Ok(info) = self.rpc_client.test_connection() {
            self.node_info = info;
            self.last_update = Instant::now();
        }
        Ok(())
    }
}

fn format_timestamp(timestamp: i64) -> String {
    let dt: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

impl Drop for UI {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).unwrap();
    }
} 