# Projektstruktur

## Verzeichnisse

```
nodebridge/
├── src/
│   ├── rpc/                 # Bitcoin Core RPC Kommunikation
│   │   ├── mod.rs          # RPC Client & Basistypen
│   │   ├── mempool.rs      # Mempool-spezifische RPC Calls
│   │   ├── mining.rs       # Mining & Difficulty RPC Calls (NEU)
│   │   └── security.rs     # Sicherheits-bezogene Calls (NEU)
│   │
│   ├── ui/                 # Terminal UI Komponenten
│   │   ├── mod.rs         # UI State & Event Handling
│   │   ├── tabs/          # Tab-spezifische UI Komponenten
│   │   │   ├── overview.rs # Übersicht & Block Tab
│   │   │   ├── mempool.rs  # Mempool Visualisierung
│   │   │   ├── network.rs  # Netzwerk & Peers
│   │   │   ├── mining.rs   # Mining Informationen (NEU)
│   │   │   └── security.rs # Sicherheits Tab (NEU)
│   │   │
│   │   ├── components.rs  # Wiederverwendbare UI Komponenten
│   │   ├── theme.rs       # Farben & Styling (NEU)
│   │   ├── help.rs        # Hilfe-System
│   │   └── render.rs      # Haupt-Rendering Logik
│   │
│   ├── config/            # Konfiguration (NEU)
│   │   ├── mod.rs        # Konfigurations-Management
│   │   └── settings.rs    # Benutzereinstellungen
│   │
│   └── main.rs           # Programmstart & Setup
│
├── config/               # Konfigurationsdateien (NEU)
│   └── default.toml     # Standard-Konfiguration
│
└── docs/                # Dokumentation (NEU)
    ├── MASTERPLAN.md    # Projektübersicht
    ├── STRUKTUR.md      # Diese Datei
    └── FORTSCHRITT.md   # Entwicklungsfortschritt
```

## Modulare Struktur

### RPC Module
- **mod.rs**: Basis RPC Client & gemeinsame Typen
- **mempool.rs**: Mempool-bezogene RPC Calls
- **mining.rs**: Mining & Difficulty Abfragen (NEU)
- **security.rs**: Sicherheitsrelevante Abfragen (NEU)

### UI Module
- **tabs/**: Separate Module für jeden Tab
  - Bessere Wartbarkeit
  - Klare Zuständigkeiten
  - Einfachere Tests

- **components.rs**: Gemeinsame UI Elemente
  - Header
  - Footer
  - Navigation
  - Status-Anzeigen
  - Popups & Alerts

- **theme.rs**: Theming System (NEU)
  - Farbschemata
  - Styling Definitionen
  - Dark/Light Mode

### Konfiguration (NEU)
- Zentrale Konfigurationsverwaltung
- Benutzereinstellungen
- Persistente Speicherung

## Datenfluss
1. RPC Client sammelt Daten
2. Daten werden aufbereitet und zwischengespeichert
3. UI Module rendern Daten
4. Event System verarbeitet Benutzereingaben
5. Konfiguration steuert Verhalten

## Erweiterbarkeit
- Modulare Struktur ermöglicht einfache Erweiterungen
- Neue Tabs können unabhängig entwickelt werden
- Gemeinsame Komponenten fördern Konsistenz