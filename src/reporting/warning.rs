use serde::Serialize;

#[derive(Serialize)]
pub struct Warning {
    pub message: String,
    pub absolute_path: String,
}
