use serde::{Deserialize, Serialize};

static GROUPS: &str = include_str!("../../communities.json");

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub name: String,
    pub about: String,
    pub telegram: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Groups {
    groups: Vec<Group>,
}

impl Groups {
    pub fn new() -> Self {
        let json: Vec<Group> = serde_json::from_str(GROUPS).unwrap();

        Self { groups: json }
    }

    pub fn get_groups(&self, page_number: i32, page_size: i32) -> Vec<Group> {
        let start = (page_number - 1) * page_size;
        let end = page_number * page_size;

        println!("start: {}, end: {}", start, end);

        if start < 0 || end < 0 {
            return Vec::new();
        }

        if start as usize > self.groups.len() {
            return Vec::new();
        }

        if end as usize > self.groups.len() {
            return self.groups[start as usize..].to_vec();
        }

        self.groups[start as usize..end as usize].to_vec()
    }
}
