set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Toolchain: rust-toolchain.toml. Extra tools: cargo-nextest, just, and the
# lele-rust-linter binaries (`cargo install --git .../lele-rust-linter lele-lint`).

# Headless Bevy previews render deterministically (and tear down cleanly) on the
# software Vulkan driver; fall back to the system default when lavapipe is absent.
_lavapipe := `ls /usr/share/vulkan/icd.d/lvp_icd*.json 2>/dev/null | head -n1`

build:
    cargo build --all-targets --all-features

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt -- --check

test:
    VK_ICD_FILENAMES={{ _lavapipe }} cargo nextest run --all-targets --all-features

lint:
    lele-lint crates/telegram-bot-helper
    lele-lint crates/telegram-bot-helper-macros
    lele-lint crates/lele-bevy-preview

bevy-lint:
    lele-bevy-lint crates/lele-bevy-preview

setup:
    prek install -t pre-commit -t pre-push

check:
    prek run --all-files

ci: build check
