![Work Tracker build pipeline](https://github.com/wagnrd/work-tracker/actions/workflows/build-and-release.yml/badge.svg)

# Work Tracker
A useful app to easily keep track of your work hours. Developed in Rust with Gtk4 via Relm4.


## Features
- [ ] Manually track start time, end time and breaks
- [ ] Show total and average work/break time of your week
- [ ] Show average work/break time per day and week
- [ ] Automatically track work time
- [ ] Show history of work/break time
- [ ] Manually track focsed work
- [ ] Semi-automatically track focused work/break time via lock-screen and duration thresholds
- [ ] Sync to online account to share data between PCs

## Platform support
- GNU/Linux
- macOS

(Currently no future plans for Windows)

## Building from source
### Dependencies
- Rust 1.69+
- Gtk4 4.10+ (with GLib >= 2.74)
- Libadwaita 1.2+

### Installation
Install Rust via [Rustup](https://rustup.rs/).

#### Linux
Fedora/Red Hat:
```shell
sudo dnf install gtk4-devel gcc libadwaita-devel
```
Debian/Ubuntu:
```shell
sudo apt install libgtk-4-dev build-essential libadwaita-1-dev
```
Arch:
```shell
sudo pacman -S gtk4 base-devel libadwaita
```

#### macOS
```shell
brew install gtk4 libadwaita
```

### Build
```shell
cargo build -r
```

### Run with debug
```shell
cargo run
```
