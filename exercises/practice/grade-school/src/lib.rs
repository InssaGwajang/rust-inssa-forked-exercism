use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    database: BTreeMap<u32, BTreeSet<String>>,
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}

impl School {
    pub fn new() -> School {
        School {
            database: BTreeMap::new(),
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
        // self.database.keys().map(|grade| grade.clone()).collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.database.get(&grade) {
            Some(grade) => grade.iter().cloned().collect(),
            // Some(grade) => grade.iter().map(|student| student.clone()).collect(),
            _ => Vec::new(),
        }
    }
}

// struct Student {
//     grade: u32,
//     name: String,
// }

// impl Student {
//     fn new(grade: u32, student: &str) -> Self {
//         Self {
//             grade,
//             name: String::from(student),
//         }
//     }
// }

// pub struct School {
//     students: Vec<Student>,
// }

// impl Default for School {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl School {
//     pub fn new() -> School {
//         School {
//             students: Vec::<Student>::new(),
//         }
//     }

//     pub fn add(&mut self, grade: u32, student: &str) {
//         self.students.push(Student::new(grade, student));
//     }

//     pub fn grades(&self) -> Vec<u32> {
//         let mut grades = self
//             .students
//             .iter()
//             .fold(Vec::<u32>::new(), |mut grades, student| {
//                 if !grades.contains(&student.grade) {
//                     grades.push(student.grade);
//                 }
//                 grades
//             });

//         grades.sort_unstable();
//         grades
//     }

//     pub fn grade(&self, grade: u32) -> Vec<String> {
//         let mut names = self
//             .students
//             .iter()
//             .filter(|students| students.grade == grade)
//             .fold(Vec::<String>::new(), |mut names, student| {
//                 names.push(student.name.clone());
//                 names
//             });

//         names.sort();
//         names
//     }
// }
