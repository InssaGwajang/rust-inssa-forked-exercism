use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    database: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            ..Default::default()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.database
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        self.database.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.database.get(&grade) {
            Some(grade) => grade.iter().cloned().collect(),
            _ => Vec::new(),
        }
    }
}
