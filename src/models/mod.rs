#[derive(Debug)]

pub struct Anime {
    name: String,
    age: i32,
    category: String,
    year: i32,
}

impl Anime {
    pub fn new(name: String, age: i32, category: String, year: i32) -> Self {
        Self {
            name,
            age,
            category,
            year,
        }
    }

    pub fn anime_description(&self) -> String {
        format!(
            "{}, {}, {}, {},",
            self.name, self.age, self.category, self.year
        )
    }

    pub fn set_anime_name(&mut self, name: String) {
        self.name = name
    }
}
