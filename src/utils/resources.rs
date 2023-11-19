use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static RESOURCE: &'static str = include_str!("../../source.json");

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub name: String,
    pub desc: String,
    pub link: String,
}

#[derive(Clone, Debug)]
pub struct Resources {
    materials: HashMap<String, Vec<Resource>>,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            materials: serde_json::from_str(RESOURCE).unwrap(),
        }
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.materials.keys().map(|k| k.to_string()).collect()
    }

    pub fn get_materials(&self, key: &str) -> Option<&Vec<Resource>> {
        self.materials.get(key)
    }

    pub fn get_material(&self, key: &str, index: usize) -> Option<&Resource> {
        self.materials.get(key).and_then(|v| v.get(index))
    }
}
