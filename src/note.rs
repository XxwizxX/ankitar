use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Note(pub String);

impl Note {
    pub fn new(name: String) -> Self {
        Note(name)
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.0 == other.0
    }
}
