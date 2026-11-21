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

# Re-send every UI preview to Telegram, ignoring the change gate.
resend-all-previews:
    LELE_PREVIEW_RESEND_ALL=1 VK_ICD_FILENAMES={{ _lavapipe }} cargo nextest run ui_png_preview ui_mp4_preview ui_scene_preview --all-features

lint:
    lele-lint --config lele.toml --sync-config --check crates/telegram-bot-helper
    lele-lint crates/telegram-bot-helper
    lele-lint crates/telegram-bot-helper-macros
    lele-lint crates/lele-bevy-preview

bevy-lint:
    lele-bevy-lint --config lele.toml --sync-config --check crates/lele-bevy-preview
    lele-bevy-lint crates/lele-bevy-preview

setup:
    prek install -t pre-commit -t pre-push -t commit-msg

[doc('commit-msg hook: a commit that changes a real lele.toml must carry a Lele-Config-Request: trailer (the user request it implements).')]
lele-config-guard MSG_FILE:
    #!/usr/bin/env bash
    set -euo pipefail
    changed=$(git diff --cached --name-only | grep -E '(^|/)lele\.toml$' | grep -vE '(^|/)(lele-lint-test-crates|rule_examples|interactions|tests/fixtures)/' || true)
    [ -z "$changed" ] && exit 0
    echo "lele.toml changed in this commit:" >&2
    git diff --cached --stat -- $changed >&2
    git diff --cached -- $changed | grep -E '^[+-][^+-]' | grep -E 'level|reason|exempt|path|replaced_by|\[' >&2 || true
    if ! grep -qE '^Lele-Config-Request: .+' "{{ MSG_FILE }}"; then
        echo "lele.toml changed: add a 'Lele-Config-Request: <what the user asked>' trailer. Agents: only when the user asked for this change in this conversation." >&2
        exit 1
    fi

check:
    prek run --all-files

ci: build check
