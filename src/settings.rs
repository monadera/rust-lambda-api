use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub table_name: String,
}
