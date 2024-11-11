use std::collections::HashSet;

pub struct School {
    grades: Vec<HashSet<String>>,
}

impl School {
    pub fn new() -> School {
        Self { grades: Vec::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.grades.len() <= grade as usize {
            self.grades
                .resize_with(grade as usize + 1, Default::default);
        }

        if self.grades.iter().any(|grade| grade.contains(student)) {
            return;
        }

        self.grades[grade as usize].insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades
            .iter()
            .enumerate()
            .filter_map(|(i, grade)| {
                if !grade.is_empty() {
                    Some(i as u32)
                } else {
                    None
                }
            })
            .collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(grade as usize)
            .map(|grade| {
                let mut students = grade.iter().cloned().collect::<Vec<_>>();
                students.sort();
                students
            })
            .unwrap_or_default()
    }
}
