# telegram-bot-helper

Blocking Telegram Bot API senders for debug tooling, plus the deterministic
headless Bevy UI preview tool that uses them.

| Crate | Role |
|---|---|
| `telegram-bot-helper` | `telegram-bot-helper` lib — text/photo/video senders, test reporting |
| `telegram-bot-helper-macros` | `#[telegram_notify]` attribute macro |
| `lele-bevy-preview` (`lele_bevy_preview`) | headless PNG/MP4 previews with change-gated Telegram delivery |

## Dependencies and releases

The three crates in this repo share one version and one tag. They depend on the
sibling repository `lele-rust-linter` (`atomic-delegate-macros`), pinned in the
workspace manifest:

    atomic-delegate-macros = { git = "https://github.com/Uomocosa/lele-rust-linter", tag = "v0.0.3" }

A git content filter (`lele-pins`) rewrites those pins to sibling `path =`
dependencies in your working tree, so an edit in a dependency is picked up by the
next `cargo build` here with no release and no command. Install it once per
machine:

    cd ../lele-release && just setup

Never hand-edit a pin. From the fleet root, `just release <patch|minor|major>`
bumps every changed repo, re-pins its dependents, tags `vX.Y.Z` and pushes; see
`lele-release/README.md` for `status`, `--only`/`--exclude` and `--dry-run`. A
plain clone without the filter resolves the committed tags, which is exactly what
CI sees.

## Environment

Install the toolchain and dev tools — no nix, no container, no setup script:

```sh
rustup toolchain install nightly    # rust-toolchain.toml pins channel + components
cargo install cargo-nextest
cargo install just
cargo install --git https://github.com/Uomocosa/lele-rust-linter lele-lint
cargo binstall prek                 # hook runner (`prek install` via `just setup`)
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
just setup    # one-time per clone: install the prek git hooks
just check    # run the full gate (same as CI)
just ci
```
