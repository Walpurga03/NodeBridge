pub mod tabs;
mod help;
mod components;
mod render;
mod common;

use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend},
    Terminal,
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    layout::Alignment,
};
use crate::rpc::{BitcoinRPC, NodeStatus};
pub use crate::ui::tabs::block_details::BlockSearchMode;
use crate::ui::tabs::tx_details::TxMode;
use crate::ui::tabs::address_details::AddressMode;

pub(crate) use common::*;

#[derive(PartialEq)]
pub enum Tab {
    Dashboard,
    BlockDetails,
    TxDetails,
    AddressDetails,
    Mempool,
    Network,
    PeerList,
    Mining,
    Security,
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    current_tab: Tab,
    show_help: bool,
    update_interval: Duration,
    last_update: Instant,
    is_updating: bool,
    spinner_state: usize,
    rpc_client: Option<BitcoinRPC>,
    node_info: Option<NodeStatus>,
    connection_state: ConnectionState,
    status_messages: Vec<StatusMessage>,
    block_input_active: bool,
    block_input: String,
    block_search_mode: BlockSearchMode,
    tx_mode: Option<TxMode>,
    address_mode: Option<AddressMode>,
    input: Option<String>,
    last_status_time: Instant,
}

#[derive(Clone)]
pub struct StatusMessage {
    pub text: String,
    pub level: MessageLevel,
    pub timestamp: Instant,
}

#[derive(Clone, PartialEq)]
pub enum MessageLevel {
    Info,
    Warning,
    #[allow(dead_code)]
    Error,
}

impl UI {
    pub fn new(initial_block_mode: BlockSearchMode, 
               initial_tx: Option<String>,
               initial_addr: Option<String>) -> anyhow::Result<Self> {
        // Terminal in Raw-Mode versetzen
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnableMouseCapture)?;
        
        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        
        Ok(Self {
            terminal,
            current_tab: Tab::Dashboard,
            show_help: false,
            update_interval: Duration::from_secs(10),
            last_update: Instant::now(),
            is_updating: false,
            spinner_state: 0,
            rpc_client: None,
            node_info: None,
            connection_state: ConnectionState::Connecting,
            status_messages: Vec::new(),
            block_input_active: false,
            block_input: String::new(),
            block_search_mode: initial_block_mode,
            tx_mode: initial_tx.or(Some("bcac1259b3faf4d01f8f0d99d5340576f197553a899e058ea3833fe5f82e0345".to_string()))
                .map(|txid| TxMode { txid }),
            address_mode: initial_addr.or(Some("bc1p38hzyl8p5yyqnzgkcxttr6ac0wc0ae8gpv7rld79df88qkrva38s78e8wd".to_string()))
                .map(|address| AddressMode { address }),
            input: None,
            last_status_time: Instant::now(),
        })
    }

    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        // Terminal-Modus zurücksetzen
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    fn try_connect(&mut self) -> anyhow::Result<()> {
        self.connection_state = ConnectionState::Connecting;
        self.show_status("Verbinde mit Bitcoin Node...".to_string(), MessageLevel::Info);
        
        match BitcoinRPC::new() {
            Ok(client) => {
                match client.test_connection() {
                    Ok(info) => {
                        self.rpc_client = Some(client);
                        self.node_info = Some(info);
                        self.connection_state = ConnectionState::Connected;
                        self.last_update = Instant::now();
                        self.show_status("Verbindung hergestellt".to_string(), MessageLevel::Info);
                    }
                    Err(e) => {
                        let error = format!("Verbindungstest fehlgeschlagen: {}", e);
                        self.connection_state = ConnectionState::Error(error.clone());
                        self.show_status(error, MessageLevel::Error);
                    }
                }
            }
            Err(e) => {
                let error = format!("RPC-Client Fehler: {}", e);
                self.connection_state = ConnectionState::Error(error.clone());
                self.show_status(error, MessageLevel::Error);
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.try_connect()?;

        loop {
            // Alte Nachrichten entfernen
            self.cleanup_old_messages();

            // Verbindungsstatus prüfen und anzeigen
            self.terminal.draw(|f| {
                match self.connection_state {
                    ConnectionState::Connecting => {
                        let loading = Paragraph::new("Verbinde mit Bitcoin Node...")
                            .block(Block::default().borders(Borders::ALL))
                            .style(Style::default().fg(Color::Yellow))
                            .alignment(Alignment::Center);
                        f.render_widget(loading, f.size());
                    }
                    ConnectionState::Error(ref error) => {
                        let error_msg = Paragraph::new(format!("Verbindungsfehler:\n{}", error))
                            .block(Block::default().borders(Borders::ALL))
                            .style(Style::default().fg(Color::Red))
                            .alignment(Alignment::Center);
                        f.render_widget(error_msg, f.size());
                    }
                    ConnectionState::Connected => {
                        if let Some(ref node_info) = self.node_info {
                            // Normale UI rendern
                            render::draw_ui(
                                f,
                                &self.current_tab,
                                self.show_help,
                                node_info.version,
                                node_info.height,
                                &node_info.block_hash,
                                node_info.timestamp,
                                node_info.connections,
                                node_info.verification_progress,
                                node_info.mempool_size,
                                &node_info.network,
                                self.update_interval,
                                self.is_updating,
                                self.spinner_state,
                                &node_info.mempool_info,
                                &self.status_messages,
                                node_info,
                                &self.rpc_client,
                                &self.block_search_mode,
                                self.block_input_active,
                                &self.tx_mode,
                                &self.address_mode,
                            )
                        }
                    }
                }
            })?;

            // Wenn verbunden, normale Updates durchführen
            if self.connection_state == ConnectionState::Connected {
                if self.last_update.elapsed() >= self.update_interval {
                    if let Err(e) = self.refresh_data() {
                        self.connection_state = ConnectionState::Error(format!("Update fehlgeschlagen: {}", e));
                    }
                }
            }

            // Event handling
            if crossterm::event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') if self.connection_state == ConnectionState::Connected => 
                            self.refresh_data()?,
                        KeyCode::Char('c') if self.connection_state != ConnectionState::Connected => 
                            self.try_connect()?,
                        KeyCode::Char('h') => self.show_help = !self.show_help,
                        KeyCode::Char('+') => self.increase_update_interval(),
                        KeyCode::Char('-') => self.decrease_update_interval(),
                        KeyCode::Char('1') => self.current_tab = Tab::Dashboard,
                        KeyCode::Char('2') => self.current_tab = Tab::BlockDetails,
                        KeyCode::Char('3') => self.current_tab = Tab::TxDetails,
                        KeyCode::Char('4') => self.current_tab = Tab::AddressDetails,
                        KeyCode::Char('5') => self.current_tab = Tab::Mempool,
                        KeyCode::Char('6') => self.current_tab = Tab::Network,
                        KeyCode::Char('7') => self.current_tab = Tab::PeerList,
                        KeyCode::Char('8') => self.current_tab = Tab::Mining,
                        KeyCode::Char('9') => self.current_tab = Tab::Security,
                        KeyCode::Enter if self.current_tab == Tab::BlockDetails => {
                            if self.block_input_active {
                                if !self.block_input.is_empty() {
                                    self.block_search_mode = BlockSearchMode::Custom(self.block_input.clone());
                                }
                                self.block_input_active = false;
                            } else {
                                self.block_input_active = true;
                                self.block_input.clear();
                                self.block_search_mode = BlockSearchMode::Latest;
                            }
                        },
                        KeyCode::Esc => {
                            if self.current_tab == Tab::BlockDetails && self.block_input_active {
                                // Im Block-Details Tab: Eingabemodus beenden
                                self.block_input_active = false;
                                self.block_input.clear();
                                self.block_search_mode = BlockSearchMode::Latest;
                            } else {
                                // In anderen Tabs: Hilfe ausblenden
                                self.show_help = false;
                            }
                        },
                        KeyCode::Char(c) if self.block_input_active => {
                            println!("Char input: {}", c); // Debug-Ausgabe
                            self.block_input.push(c);
                        },
                        KeyCode::Backspace if self.block_input_active => {
                            self.block_input.pop();
                        },
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn refresh_data(&mut self) -> anyhow::Result<()> {
        self.is_updating = true;
        self.spinner_state = (self.spinner_state + 1) % 4;

        let result = if let Some(client) = &self.rpc_client {
            match client.test_connection() {
                Ok(info) => {
                    self.node_info = Some(info);
                    self.last_update = Instant::now();
                    Ok(())
                }
                Err(e) => {
                    self.connection_state = ConnectionState::Error(e.to_string());
                    Err(e)
                }
            }
        } else {
            Ok(())
        };

        self.is_updating = false;  // Status zurücksetzen
        result
    }

    fn increase_update_interval(&mut self) {
        let secs = self.update_interval.as_secs();
        if secs < 60 {  // Maximum 60 Sekunden
            self.update_interval = Duration::from_secs(secs + 1);
        }
    }

    fn decrease_update_interval(&mut self) {
        let secs = self.update_interval.as_secs();
        if secs > 1 {  // Minimum 1 Sekunde
            self.update_interval = Duration::from_secs(secs - 1);
        }
    }

    fn show_status(&mut self, text: String, level: MessageLevel) {
        self.status_messages.push(StatusMessage {
            text,
            level,
            timestamp: Instant::now(),
        });
    }

    // Status-Nachrichten aufräumen
    fn cleanup_old_messages(&mut self) {
        self.status_messages.retain(|msg| {
            msg.timestamp.elapsed() < Duration::from_secs(5)  // 5 Sekunden anzeigen
        });
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

#[derive(PartialEq)]
enum ConnectionState {
    Connecting,
    Connected,
    Error(String),
} 