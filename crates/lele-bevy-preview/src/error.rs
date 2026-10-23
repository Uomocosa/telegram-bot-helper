use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("preview file i/o failed")]
    Io(#[from] std::io::Error),

    #[error("preview manifest could not be serialised")]
    Json(#[from] serde_json::Error),

    #[error("cannot open image {}", path.display())]
    Image {
        path: std::path::PathBuf,
        #[source]
        source: image::ImageError,
    },

    #[error("preview scene is invalid: {0}")]
    Scene(String),

    #[error("no frame was captured within {0} updates")]
    NoFrame(u32),

    #[error("captured frame is empty: {0}")]
    BlankFrame(String),

    #[error(
        "captured frame at {path} has only {distinct} distinct colour(s); expected at least {min} (nothing rendered?)"
    )]
    NoVisual {
        path: String,
        distinct: usize,
        min: usize,
    },

    #[error(
        "clip `{label}` of scene `{scene}` is static: every frame is identical, so it shows no change"
    )]
    StaticClip { scene: String, label: String },

    #[error("ffmpeg is required to build an mp4 preview but was not found on PATH")]
    NoFfmpeg,

    #[error("ffmpeg failed for {path}: {message}")]
    Ffmpeg { path: String, message: String },

    #[error("ffmpeg could not be started for {path}")]
    FfmpegSpawn {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error(
        "capture {path} is not centred: content centre is ({x}, {y}), the frame centre is ({expected_x}, {expected_y}), tolerance {tolerance}px"
    )]
    NotCentered {
        path: String,
        x: u32,
        y: u32,
        expected_x: u32,
        expected_y: u32,
        tolerance: u32,
    },

    #[error("capture {0} shows only background, so there is nothing to centre")]
    NoContent(String),

    #[error("telegram delivery failed")]
    Telegram(#[from] telegram_bot_helper::Error),
}
// no test_usage necessary
