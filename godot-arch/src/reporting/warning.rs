use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Warning {
    pub message: String,
    pub absolute_path: String,
}
