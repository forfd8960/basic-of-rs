#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub country: String,
    pub city: String,
}

impl Person {
    pub fn new(name: String, age: u8, loc: Location) -> Self {
        Self {
            name,
            age,
            location: loc,
        }
    }

    pub fn come_from_same_place(&self, other: Person) -> bool {
        self.location.eq(&other.location)
    }
}
