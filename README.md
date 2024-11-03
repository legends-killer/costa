> iOS simulator cli wrapper for h5, mini program debugging.

# iOS Simulator Debugger

A desktop application that provides a convenient interface for managing iOS simulators and apps for H5 and mini program debugging.

## Features

- Install and manage Xcode command line tools
- Download and install iOS Simulators
- View and delete installed simulator runtimes 
- Download and install test apps to simulators
- Interact with simulators via CLI wrapper
- Interact with apps via HTTP API for debugging

## Installation

1. Download the latest release for your platform
2. Install and run the application
3. Follow the setup wizard to install required dependencies

## Usage

### Xcode Tools Setup
- Click "Install Xcode Toolchains" to install required command line tools
- Follow terminal prompts to complete installation

### Simulator Management 
- Use "Download Simulator" to get the latest iOS simulator
- Click "Install Simulator" to install downloaded simulator packages
- View installed simulators and their status
- Delete unwanted simulator runtimes

### App Installation
- Download test apps using the provided buttons
- View downloaded apps in the app list
- Install apps to simulator with one click
- Debug apps via HTTP API **(additional modification in app code is required)**

### How It Works
- calling debug API from App directly. see [HostOperation](./src-tauri/src/host/host.rs)
- invoke universal link from App to simulator. see [Apple UniversalLink](https://developer.apple.com/documentation/xcode/allowing-apps-and-websites-to-link-to-your-content/)

## Development

This app is built with:
- Tauri (Rust + TypeScript)
- React
- Arco Design UI Components

### Setup

#### Initialize repo
```bash
chmod +x ./init.sh
./init.sh
```

#### Install dependencies
```bash
yarn
```

#### Run React App
```bash
yarn dev
```

#### Run Tauri App
```bash
yarn tauri dev
```

### Build

#### Build React App First
```bash
yarn build
```

#### Build Tauri App
```bash
yarn tauri build-app-arm64 #for macos arm64
yarn tauri build-app-x64 #for macos x64
yarn tauri build-debug-app-arm64 #for macos arm64 debug
yarn tauri build-debug-app-x64 #for macos x64 debug
```

## License

MIT License
