# notifycli

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

A lightweight, portable, single-binary CLI tool for sending native desktop notifications on Linux and Windows.

`notifycli` creates no files or cache directories on host systems at runtime and operates entirely in-memory using native OS notification protocols (D-Bus on Linux, WinRT/Toast API on Windows).

---

## Features

- Single Binary: Zero runtime external dependencies.
- Maximum Optimization: Compiled with LTO, opt-level "z", panic abort, and symbols stripped (~1.3 MB).
- Zero File Footprint: Creates no temporary files, cache, or config files on your system.
- Notification Pinning: `--pin` flag keeps a notification resident until dismissed by the user.
- Universal ASCII Progress Bar: Using `-p, --progress <0-100>` automatically renders a visual progress bar (`[███████████████░░░░░] 75%`) inside the notification body across all desktop environments (KDE Plasma, GNOME, Windows, Dunst, etc.).
- Notification Updates & Progression:
  - `--print-id`: Outputs the notification ID to `stdout` when created.
  - `-r, --replace-id <ID>`: Updates an existing notification in-place without triggering a new popup.
  - `-d, --delay <MS>`: Adds an optional delay in milliseconds to prevent KDE Plasma `ExcessNotificationGeneration` rate-limit errors.
- Desktop Integration & Hints:
  - `--desktop-entry`: Associate notifications with a specific `.desktop` application (e.g. `org.kde.dolphin`, `firefox`) for grouping in system tray.
  - `--origin-name`: Specify custom origin metadata.
  - `--hint <TYPE:NAME:VALUE>`: Pass arbitrary custom D-Bus hints dynamically.
- Urgency & Categories: Set notification urgency (`low`, `normal`, `critical`) and categories (`email`, `transfer`, etc.).
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
| `--pin` | | Pin notification persistently until user dismisses it | `false` |
| `--icon <ICON>` | `-i` | System icon name or file path | `None` |
| `--progress <PERCENT>` | `-p` | Progress percentage (`0` to `100`), auto-appends ASCII bar | `None` |
| `--no-bar` | | Disable automatic ASCII progress bar appending | `false` |
| `--print-id` | | Print notification ID to stdout after sending | `false` |
| `--replace-id <ID>` | `-r` | Replace / update existing notification by ID | `None` |
| `--delay <MS>` | `-d` | Delay execution in ms before sending | `0` |
| `--desktop-entry <ENTRY>` | | Associate with desktop entry ID for KDE Plasma grouping | `None` |
| `--origin-name <NAME>` | | Origin name hint (`x-kde-origin-name`) | `None` |
| `--hint <TYPE:NAME:VAL>` | | Arbitrary custom D-Bus hint (`int:name:val`, `string:name:val`) | `[]` |
| `--urgency <LEVEL>` | `-u` | Urgency level (`low`, `normal`, `critical`) | `"normal"` |
| `--category <CAT>` | `-c` | Category hint (e.g. `transfer`, `email`, `device`) | `None` |
| `--help` | `-h` | Print help information | |
| `--version` | `-V` | Print version information | |

---

## Examples

#### Quick Progress Bar Test Command
Run this one-liner to test animated progress bar updates in real time:

```bash
NID=$(notifycli -s "System Download" -b "Initializing..." -p 0 --print-id) && for i in {10..100..10}; do sleep 0.8; notifycli -s "System Download" -b "Downloading update..." -p $i -r $NID; done && notifycli -s "Download Complete!" -b "File installed successfully." -p 100 -r $NID
```

#### Pinned Notification (Stays Until Dismissed)
```bash
notifycli -s "Important Notice" -b "This notification stays pinned until closed." --pin
```

#### Single Progress Bar Notification
```bash
notifycli -s "Downloading Archive" -b "Downloading files..." -p 75
```

---

## License

Distributed under the GNU General Public License v3.0 (GPL-3.0). See [`LICENSE`](LICENSE) for more information.
