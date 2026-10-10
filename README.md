# telegram-bot-helper

Blocking Telegram Bot API senders for debug tooling, plus the deterministic
headless Bevy UI preview tool that uses them.

| Crate | Role |
|---|---|
| `telegram-bot-helper` | `telegram-bot-helper` lib — text/photo/video senders, test reporting |
| `telegram-bot-helper-macros` | `#[telegram_notify]` attribute macro |
| `lele-bevy-preview` (`lele_bevy_preview`) | headless PNG/MP4 previews with change-gated Telegram delivery |

## Environment

Install the toolchain and dev tools — no nix, no container, no setup script:

```sh
rustup toolchain install nightly    # rust-toolchain.toml pins channel + components
cargo install cargo-nextest
cargo install just
cargo install --git https://github.com/Uomocosa/lele-rust-linter lele-lint
```

System libraries — needed only to build/run `lele-bevy-preview` (Bevy) and record
MP4s:

| OS | Install |
|---|---|
| Linux | `sudo apt install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev libx11-dev libxcursor-dev libxi-dev libxrandr-dev mesa-vulkan-drivers ffmpeg` |
| macOS | `brew install ffmpeg` (Metal/Cocoa/CoreAudio are native) |
| Windows | `winget install Gyan.FFmpeg` (DX12/Vulkan/WASAPI are native) |

Telegram delivery reads credentials from `.env` (`TELEGRAM_*`) — never commit it.

## Tasks

```sh
just build | just clippy | just fmt | just test | just lint
just hooks      # point git at .githooks/
just ci
```
