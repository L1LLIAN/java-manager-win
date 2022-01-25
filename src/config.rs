use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub jvm_paths: Vec<String>,
    pub current_path: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            jvm_paths: Vec::new(),
            current_path: None
        }
    }
}

