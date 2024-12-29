use crate::ui::common::*;
use num_format::{Locale, ToFormattedString};

pub struct RPCStatus {
    pub ssl_enabled: bool,
    pub ip_restricted: bool,
    pub auth_required: bool,
    pub command_restrictions: bool,
}

pub struct SecurityStatus {
    pub version: u64,
    pub latest_version: u64,
    pub peers_total: u64,
    pub peers_onion: u64,
    pub peers_clearnet: u64,
    pub firewall_active: bool,
    pub tor_active: bool,
    pub rpc_restricted: bool,
    pub wallet_encrypted: bool,
    pub disk_encryption: bool,
    pub uptime: u64,
    pub last_backup: Option<i64>,
    pub rpc_status: RPCStatus,
}

pub fn render(status: &SecurityStatus) -> Paragraph<'static> {
    let content = vec![
        Line::from(vec![
            Span::styled("🔒 Sicherheitsstatus", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📦 Software", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Version: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("v{}", status.version),
                Style::default().fg(if status.version >= status.latest_version {
                    Color::Green
                } else {
                    Color::Red
                })
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔌 Netzwerk", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Verbindungen: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} ({} Tor / {} Direkt)", 
                    status.peers_total.to_formatted_string(&Locale::de),
                    status.peers_onion.to_formatted_string(&Locale::de),
                    status.peers_clearnet.to_formatted_string(&Locale::de)
                ),
                Style::default().fg(if status.peers_onion > status.peers_clearnet {
                    Color::Green
                } else {
                    Color::Yellow
                })
            ),
        ]),
        Line::from(vec![
            Span::styled("Firewall: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                if status.firewall_active { "Aktiv" } else { "Inaktiv" },
                Style::default().fg(if status.firewall_active {
                    Color::Green
                } else {
                    Color::Red
                })
            ),
        ]),
        Line::from(vec![
            Span::styled("Tor: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                if status.tor_active { "Aktiv" } else { "Inaktiv" },
                Style::default().fg(if status.tor_active {
                    Color::Green
                } else {
                    Color::Yellow
                })
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔐 Zugriffskontrolle", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("RPC Zugriff: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                if status.rpc_restricted { "Eingeschränkt" } else { "Offen" },
                Style::default().fg(if status.rpc_restricted {
                    Color::Green
                } else {
                    Color::Red
                })
            ),
        ]),
        Line::from(vec![
            Span::styled("Wallet: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                if status.wallet_encrypted { "Verschlüsselt" } else { "Unverschlüsselt" },
                Style::default().fg(if status.wallet_encrypted {
                    Color::Green
                } else {
                    Color::Red
                })
            ),
        ]),
        Line::from(vec![
            Span::styled("Festplatte: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                if status.disk_encryption { "Verschlüsselt" } else { "Unverschlüsselt" },
                Style::default().fg(if status.disk_encryption {
                    Color::Green
                } else {
                    Color::Yellow
                })
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⏰ System", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        ]),
        Line::from(vec![
            Span::styled("Uptime: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format_uptime(status.uptime),
                Style::default().fg(Color::White)
            ),
        ]),
        Line::from(vec![
            Span::styled("Letztes Backup: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                match status.last_backup {
                    Some(timestamp) => format_backup_time(timestamp),
                    None => "Kein Backup gefunden".to_string()
                },
                Style::default().fg(match status.last_backup {
                    Some(ts) if (chrono::Utc::now().timestamp() - ts) < 7 * 24 * 3600 => Color::Green,
                    Some(_) => Color::Yellow,
                    None => Color::Red
                })
            ),
        ]),
        Line::from(vec![
            Span::styled("RPC Sicherheit: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("SSL: {}, IP: {}, Auth: {}", 
                    if status.rpc_status.ssl_enabled { "✓" } else { "✗" },
                    if status.rpc_status.ip_restricted { "✓" } else { "✗" },
                    if status.rpc_status.auth_required { "✓" } else { "✗" }
                ),
                Style::default().fg(if status.rpc_status.ssl_enabled && 
                                     status.rpc_status.ip_restricted && 
                                     status.rpc_status.auth_required {
                    Color::Green
                } else {
                    Color::Red
                })
            ),
        ]),
    ];

    Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Sicherheit "))
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / (24 * 3600);
    let hours = (seconds % (24 * 3600)) / 3600;
    let minutes = (seconds % 3600) / 60;
    
    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn format_backup_time(timestamp: i64) -> String {
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0)
        .unwrap_or_default();
    dt.format("%Y-%m-%d %H:%M").to_string()
} 