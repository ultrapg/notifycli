# notifycli

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

A lightweight, portable, single-binary CLI tool for sending native desktop notifications on Linux and Windows.

`notifycli` creates no files or cache directories on host systems at runtime and operates entirely in-memory using native OS notification protocols (D-Bus on Linux, WinRT/Toast API on Windows).

---

## Features

- Single Binary: Zero runtime external dependencies.
- Lightweight: Optimized release build (~1.3 MB).
- Zero File Footprint: Creates no temporary files, cache, or config files on your system.
- Cross-Platform: Supports Linux and Windows natively.

---

## Installation

### Prerequisites
Make sure you have Cargo installed (Rust 1.70+ recommended).

### Building from Source

```bash
git clone https://github.com/ultrapg/notifycli.git
cd notifycli
cargo build --release
```

The optimized single executable binary will be available at:
```bash
./target/release/notifycli
```

---

## Usage

```bash
notifycli [OPTIONS]
```

### Options

| Option | Short | Description | Default |
|---|---|---|---|
| `--summary <SUMMARY>` | `-s` | Summary or title of the notification | `"Notification"` |
| `--body <BODY>` | `-b` | Main body text | `""` |
| `--app-name <APP_NAME>` | `-a` | Application name displaying the notification | `"notifycli"` |
| `--timeout <MS>` | `-t` | Display timeout in ms (`0` = server default, `-1` = never) | `5000` |
| `--icon <ICON>` | `-i` | System icon name or file path | `None` |
| `--help` | `-h` | Print help information | |
| `--version` | `-V` | Print version information | |

---

## Examples

#### Basic Notification
```bash
notifycli -s "Build Complete" -b "Your project compiled successfully!"
```

#### Custom App Name and Icon
```bash
notifycli -s "Warning" -b "High disk usage detected" -a "System Monitor" -i "dialog-warning"
```

#### Custom Timeout (10 Seconds)
```bash
notifycli -s "Reminder" -b "Meeting starts in 5 minutes" -t 10000
```

---

## License

Distributed under the GNU General Public License v3.0 (GPL-3.0). See [`LICENSE`](LICENSE) for more information.
