use std::fmt::{Display, Formatter};
use crate::instrument::string::GString;
use crate::Note;

#[derive(Debug)]
pub struct Guitar {
    strings: Vec<GString>,
    fret_count: usize,
}

impl Guitar {
    pub fn standard() -> Self {
        let tuning = vec![Note::E, Note::B, Note::G, Note::D, Note::A, Note::E];
        let fret_count: usize = 12;
        let strings = tuning
            .into_iter()
            .map(|note| GString::new(note, fret_count))
            .collect();
        Guitar { strings , fret_count}
    }
}

impl Display for Guitar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "--||")?;
        for fret in 1..=self.fret_count {
            write!(f, "{:->7}", fret)?;
        }
        writeln!(f)?;
        for x in &self.strings {
            writeln!(f, "{}", x)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guitar() {
        let guitar = Guitar::standard();
        println!("{}", guitar);
    }
}