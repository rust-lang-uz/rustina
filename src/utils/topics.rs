use std::collections::HashMap;

const CONTENT: &str = include_str!("../../data/topics.json");

#[derive(serde::Deserialize)]
pub struct Content {
    name: String,
    id: Option<u32>,
}

type Contents = Vec<Content>;

#[derive(Clone, Debug)]
pub struct Topics {
    topics: HashMap<String, u32>,
}

impl Default for Topics {
    fn default() -> Self {
        Topics::new()
    }
}

impl Topics {
    pub fn new() -> Topics {
        let mut result: HashMap<String, u32> = HashMap::new();
        let content = serde_json::from_str::<Contents>(CONTENT).unwrap();

        for item in content {
            match item.id {
                Some(id) => result.insert(item.name, id),
                None => result.insert(item.name, 1),
            };
        }

        Topics { topics: result }
    }

    pub fn add(&mut self, topic: String, id: u32) {
        self.topics.insert(topic, id);
    }

    pub fn get(&self, topic: &str) -> Option<&u32> {
        self.topics.get(topic)
    }

    pub fn list(&self) -> Vec<String> {
        self.topics.keys().cloned().collect()
    }
}
