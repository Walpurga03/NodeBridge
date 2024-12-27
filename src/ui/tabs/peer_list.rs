use crate::ui::common::*;
use crate::rpc::PeerInfo;
use ratatui::widgets::Table;

pub fn render(peers: &[PeerInfo]) -> Table<'static> {
    Table::new(vec![])
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Peer Liste "))
} 