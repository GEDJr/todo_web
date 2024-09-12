use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Title {
    #[serde(rename = "Strand")]
    pub title: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CompletedStatus {
    #[serde(rename = "Bool")]
    pub status: bool,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Task {
    pub id: Value,
    pub title: Title,
    pub completed: CompletedStatus,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

#[derive(Deserialize)]
pub struct RowId {
    pub id: String ,
}