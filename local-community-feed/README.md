# Local Community Feed

A local-first, censorship-resistant social feed where communities own their own data and history.

## Building from Source

To build this project from source, you will need to have the following system dependencies installed, depending on your operating system.

### Linux (Debian/Ubuntu)

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libxdo-dev
```

### macOS

You will need to install the Xcode Command Line Tools:

```bash
xcode-select --install
```

### Windows

You will need to install the Microsoft C++ Build Tools and the WebView2 runtime. You can find instructions for this in the [official Tauri documentation](https://v2.tauri.app/start/prerequisites/#windows).

### All Platforms

Once you have the system dependencies installed, you will need to install the Node.js dependencies:

```bash
npm install
```

Then, you can run the application in development mode:

```bash
npm run tauri dev
```
