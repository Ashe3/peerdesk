# PeerDesk To-do List

## MVP Feature List

- [ ] **Electron-based UI**

  - Minimal window: manual entry of backend host (IP address or local hostname)
  - Show streamed remote screen
  - Capture and forward user mouse/keyboard to backend
  - Keep a list of recent connections for quick access

- [ ] **Rust core backend**

  - Capture and encode screen as sequence of JPEG frames (MJPEG-style streaming)
  - Serve screen stream and handle control commands (mouse/keyboard events)
  - Simple TCP/WebSocket server for Electron client connection

- [ ] **Node/Electron <-> Rust Communication**

  - Use stable socket-based protocol (WebSocket or TCP)
  - Expose both stream and command channel over same connection

- [ ] **Connection**

  - Start with manual host/IP entry
  - [Future] Add automatic LAN discovery (mDNS/UDP broadcast)

- [ ] **Basic connection flow**
  - Start backend on controlled PC
  - Run Electron client, connect to backend's host/IP and port
  - Start remote screen stream and control

## Further Goals / Roadmap

- [ ] Efficient video codec support: add H264/VP8/VP9 (via ffmpeg or gstreamer, Rust bindings)
- [ ] WebRTC transport option (peer-to-peer via DataChannel + media streams); allow selecting between WebSocket or WebRTC connection type in UI
- [ ] NAT traversal for internet access (STUN/TURN servers for WebRTC)
- [ ] Automatic LAN peer discovery
- [ ] Save favorite hosts/recent connections for easy reconnect
- [ ] Password authentication and session IDs
- [ ] File transfer (drag/drop interface, optional clipboard sync)
- [ ] Copy-and-paste between devices
- [ ] Show connection status and errors in UI
- [ ] Simple logging
- [ ] Build scripts for Windows, Linux, macOS; provide builds for all platforms
- [ ] [Long-term] Mobile clients (Android & iOS)
- [ ] Auto-update, installable packages
- [ ] Optional cloud relay server for WebRTC fallback

## Developer Notes

- MVP focus: fast remote access within LAN; minimal user setup
- WebSocket is main protocol for MVP (stability, debug-friendliness)
- WebRTC and NAT traversal are advanced, optional, and can be developed in parallel
- Easy migration: keep message formats modular — swapping between WebSocket and WebRTC DataChannel should not require major logic rewrite
