use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub table_name: String,
}
