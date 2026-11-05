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
    lele-lint crates/telegram-bot-helper
    lele-lint crates/telegram-bot-helper-macros
    lele-lint crates/lele-bevy-preview

bevy-lint:
    lele-bevy-lint crates/lele-bevy-preview

setup:
    prek install -t pre-commit -t pre-push

check:
    prek run --all-files

# List orphaned UI previews (pass `apply` to delete them).
preview-clean *ARGS:
    python3 -B scripts/preview_clean.py crates/lele-bevy-preview/ui_preview {{ ARGS }}

# Print sent lines of the newest run from a preview log.
preview-why log:
    python3 -c 'import json,sys; p=sys.argv[1]; ls=[l for l in open(p).read().splitlines() if l.strip()]; os=[]; [os.append((l,json.loads(l))) for l in ls]; r=os[-1][1].get("run") if os else None; [print(l) for l,o in os if o.get("run")==r and o.get("outcome")=="sent"]' "{{ log }}"

ci: build check
