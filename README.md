# Aurora

![Compositor
CI](https://github.com/%3Cusername%3E/aurora/actions/workflows/compositor.yml/badge.svg)
![Dock
CI](https://github.com/%3Cusername%3E/aurora/actions/workflows/dock.yml/badge.svg)
![Launcher
CI](https://github.com/%3Cusername%3E/aurora/actions/workflows/launcher.yml/badge.svg)
![Shell
CI](https://github.com/%3Cusername%3E/aurora/actions/workflows/shell.yml/badge.svg)

Aurora is a **Wayland compositor and desktop environment written in
Rust** using the [Smithay](https://github.com/smithay/smithay)
framework.

The goal of Aurora is to build a **fast, aesthetically pleasing, and
coherent Linux desktop** inspired by environments like **Pantheon,
GNOME, and macOS**.

------------------------------------------------------------------------

## Overview

Aurora provides:

-   A modern **Wayland compositor**
-   GPU accelerated rendering
-   A consistent desktop shell
-   Modular desktop components

Aurora is currently in early development.

------------------------------------------------------------------------

## Architecture

Aurora follows a layered architecture:

    Backend 
    │ 
    ├── Winit (development backend) 
    └── DRM/KMS (future production backend)

    Compositor Core 
    ├── Wayland protocol handling 
    ├── Input handling 
    ├── Window management
    └── Rendering pipeline

    Shell Layer 
    ├── Wallpaper 
    ├── Desktop icons 
    └── Window decorations

    Desktop Components 
    ├── Dock 
    ├── Launcher 
    ├── Notifications 
    └── System UI

------------------------------------------------------------------------

## Current Features

### Compositor

-   Wayland compositor built with Smithay
-   XDG shell support
-   Window move and resize
-   Popup surfaces
-   Input handling (mouse and keyboard)
-   Damage tracking
-   Frame callbacks
-   Multi-window rendering
-   Wallpaper rendering

### Rendering

-   GPU accelerated composition
-   Wayland surface rendering
-   Damage tracking
-   Output management

------------------------------------------------------------------------

## Planned Features

### Desktop Shell

-   Dock
-   Application launcher
-   Desktop icons
-   Wallpaper chooser
-   Window snapping
-   Workspace management

### System UI

-   Notifications
-   System tray
-   Power menu
-   Settings panel

### Platform

-   Multi-monitor support
-   HiDPI scaling
-   DRM backend
-   Session management

------------------------------------------------------------------------

## Project Structure

    aurora/
    │
    ├── aurora-compositor/
    │   ├── src/
    │   └── Cargo.toml
    │
    ├── aurora-dock/
    │   ├── src/
    │   └── Cargo.toml
    │
    ├── aurora-launcher/
    │   ├── src/
    │   └── Cargo.toml
    │
    ├── docs/
    │
    ├── .github/
    │   └── workflows/
    │
    └── README.md

------------------------------------------------------------------------

## Building Aurora

### Dependencies

Install required system packages:

    sudo apt install     pkg-config     libwayland-dev     libxkbcommon-dev     libudev-dev     libinput-dev     libegl1-mesa-dev     libgles2-mesa-dev     libgbm-dev     libdrm-dev     libgtk-4-dev

Install Rust:

    curl https://sh.rustup.rs -sSf | sh

------------------------------------------------------------------------

### Build

    cargo build

------------------------------------------------------------------------

### Run

    cargo run

This runs Aurora using the **winit backend**.

------------------------------------------------------------------------

## Roadmap

### Phase 1 -- Compositor Core

-   [x] Wayland compositor
-   [x] Input handling
-   [x] Window management
-   [x] Rendering pipeline
-   [x] Wallpaper

### Phase 2 -- Shell

-   [ ] Dock
-   [ ] Launcher
-   [ ] Desktop icons
-   [ ] Window decorations

### Phase 3 -- Desktop

-   [ ] Multi-monitor support
-   [ ] Workspaces
-   [ ] Notifications
-   [ ] Settings UI

------------------------------------------------------------------------

## Contributing

Contributions are welcome.

Steps:

1.  Fork repository
2.  Create a feature branch
3.  Submit a pull request

------------------------------------------------------------------------

## License

Aurora is licensed under either of:

-   MIT License
-   Apache License 2.0

at your option.

------------------------------------------------------------------------

## Credits

Aurora is built using:

-   Smithay
-   Wayland
-   Rust