use std::collections::HashMap;

pub struct School {
	grades: HashMap<usize, Vec<String>>,
}

impl School {
	pub fn new() -> School {
		School {
			grades: HashMap::new()
		}
	}
	pub fn grade(&self, year: usize) -> Option<&Vec<String>> {
		self.grades.get(&year).clone()
	}
	pub fn grades(&self) -> Vec<usize> {
		let mut grade_list: Vec<usize> = self.grades.keys().map(|k| { k.clone() }).collect();
		grade_list.sort_by(|a, b| a.cmp(b));
		grade_list
	}
	pub fn add(&mut self, year: usize, name: &str) {
		let mut grade_data = self.grades.entry(year).or_insert(vec![]);
		grade_data.push(String::from(name));
		grade_data.sort_by(|a, b| a.cmp(b));
	}
}