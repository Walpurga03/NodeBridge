mod tabs;
mod help;
mod components;
mod render;
mod common;

use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend},
    Terminal,
};
use crate::rpc::{BitcoinRPC, NodeStatus};

pub(crate) use common::*;

#[derive(PartialEq)]
pub enum Tab {
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
    show_help: bool,
    is_updating: bool,
    spinner_state: usize,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
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
            show_help: false,
            is_updating: false,
            spinner_state: 0,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if self.last_update.elapsed() >= self.update_interval {
                self.is_updating = true;
                self.spinner_state = (self.spinner_state + 1) % 4;
                
                if let Ok(info) = self.rpc_client.test_connection() {
                    self.node_info = info;
                    self.last_update = Instant::now();
                }
                
                self.is_updating = false;
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
                    self.show_help,
                    version,
                    height,
                    &block_hash,
                    timestamp,
                    connections,
                    verification_progress,
                    mempool_size,
                    &network,
                    self.update_interval,
                    self.is_updating,
                    self.spinner_state,
                    &self.node_info.mempool_info,
                )
            })?;

            if crossterm::event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => self.refresh_data()?,
                        KeyCode::Char('h') => self.show_help = !self.show_help,
                        KeyCode::Char('+') => self.increase_update_interval(),
                        KeyCode::Char('-') => self.decrease_update_interval(),
                        KeyCode::Char('1') => self.current_tab = Tab::Overview,
                        KeyCode::Char('2') => self.current_tab = Tab::BlockDetails,
                        KeyCode::Char('3') => self.current_tab = Tab::Mempool,
                        KeyCode::Char('4') => self.current_tab = Tab::Network,
                        KeyCode::Esc => self.show_help = false,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn refresh_data(&mut self) -> anyhow::Result<()> {
        if let Ok(info) = self.rpc_client.test_connection() {
            self.node_info = info;
            self.last_update = Instant::now();
        }
        Ok(())
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