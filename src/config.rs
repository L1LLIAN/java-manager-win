use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    jvm_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            jvm_paths: Vec::new(),
        }
    }
}

