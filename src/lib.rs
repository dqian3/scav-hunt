use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct CompletedTask {
    player: String,
    #[serde(with = "ts_seconds")]
    time: DateTime::<Utc>,
    image_url: String,
    notes: String,
}

#[derive(Serialize, Deserialize)]
pub enum Task {
    Incomplete,
    Complete(CompletedTask),
}
