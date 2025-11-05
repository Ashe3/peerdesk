# PeerDesk

A minimal open-source remote desktop solution for personal/private use, inspired by AnyDesk/TeamViewer.  
Tech stack: **Rust** for core screen capture/input, **Electron/Node.js** for cross-platform GUI and connectivity.

## Features (planned)

- Screen capture and streaming (LAN and WAN/internet)
- Remote mouse and keyboard control
- Simple authentication (optional for local use)
- Works on Windows/Linux/macOS (aiming for cross-platform)

## Project Structure

```
peerdesk/
├── desktop-app/      # Electron + JS user interface (client)
├── rust-core/        # Rust backend (screen, input, networking)
├── README.md
├── TODO.md
└── .gitignore
```

## Quick Start (to be completed)

1. `cd rust-core && cargo build`
2. `cd ../desktop-app && npm install && npm start`
3. Connect desktop-app to running rust-core instance.
4. For full configuration see [TODO.md](TODO.md)

## Contributing

This project is MIT-licensed. Issues and PRs are welcome!

---

```
peerdesk/
├── desktop-app/
│   └── ... (Electron JS GUI)
├── rust-core/
│   └── ... (Rust code: screen grab + input + network)
```

---
