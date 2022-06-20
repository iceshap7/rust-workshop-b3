use std::collections::HashMap;

fn main() {
    println!("\n===============");
    let mut school_a = School::new();
    school_a.add(7, "Alice");
    school_a.add(2, "Bob");
    println!("{:?}", school_a.students);

    println!("\n===============");
    let mut school_b = School::new();
    school_b.add(10, "Alice");
    school_b.add(4, "Bob");
    school_b.add(4, "Steve");
    println!("{:?}", school_b.students);
    println!("{:?}", school_b.grades());

    println!("\n===============");
    let mut school_c = School::new();
    school_c.add(3, "Alice");
    school_c.add(10, "Bob");
    school_c.add(3, "Charlie");
    println!("{:?}", school_c.students);
    println!("{:?}", school_c.grade(3));

    println!("\n===============");
    let mut school_d = School::new();
    school_d.add("A+", "Alice");
    school_d.add("B", "Bob");
    school_d.add("A+", "Charlie");
    println!("{:?}", school_d.students);
    println!("{:?}", school_d.grade("A+"));
}

pub struct School<T> {
    students: HashMap<String, T>
}

impl<T: Ord> School<T> {
    pub fn new() -> School<T> {
        School {
            students: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: T, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<T> where T: Copy {
        let mut results: Vec<T> = Vec::new();
        for (_student, _grade) in self.students.iter() {
            if !results.contains(_grade) {
                results.push(*_grade);
            }
        }

        results.sort();
        results
    }

    pub fn grade(&self, grade: T) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for (_student, _grade) in self.students.iter() {
            if *_grade == grade {
                results.push(_student.to_string());
            }
        }

        results.sort();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_school_new_with_students_empty() {
        let school_a: School<i32> = School::new();

        assert_eq!(school_a.students.len(), 0);
    }

    #[test]
    fn test_school_add_students() {
        let mut school_a = School::new();

        school_a.add(2, "Lee");
        assert_eq!(school_a.grades(), vec![2]);

        school_a.add(3, "Nancy");
        assert_eq!(school_a.grades(), vec![2, 3]);
    }

    #[test]
    fn test_students_grade() {
        let mut school_a = School::new();

        school_a.add(4, "Bob");
        school_a.add(4, "Alice");
        school_a.add(5, "Tom");

        let grade = school_a.grade(4);

        let true_results = vec!["Alice".to_string(), "Bob".to_string()];
        assert_eq!(grade, true_results);

        let false_results = vec!["Bob".to_string(), "Alice".to_string()];
        assert_ne!(grade, false_results);
    }
}
