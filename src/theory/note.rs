use clap::ValueEnum;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
#[value(rename_all = "verbatim")]
// TODO add multiple names for the same note (e.g. C# and Db)
pub enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Note {
    pub fn nth_half_step(&self, n: i32) -> Note {
        let notes = [
            Note::C,
            Note::Db,
            Note::D,
            Note::Eb,
            Note::E,
            Note::F,
            Note::Gb,
            Note::G,
            Note::Ab,
            Note::A,
            Note::Bb,
            Note::B,
        ];
        let current_index = notes.iter().position(|note| note == self).unwrap() as i32;
        let new_index = (current_index + n).rem_euclid(notes.len() as i32);
        notes[new_index as usize].clone()
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Note::C | Note::D | Note::E | Note::F | Note::G | Note::A | Note::B => {
                write!(f, "{:?}-", self)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_half_step() {
        assert_eq!(Note::C.nth_half_step(1), Note::Db);
        assert_eq!(Note::C.nth_half_step(2), Note::D);
        assert_eq!(Note::C.nth_half_step(12), Note::C);
        assert_eq!(Note::C.nth_half_step(-1), Note::B);
        assert_eq!(Note::C.nth_half_step(-2), Note::Bb);
        assert_eq!(Note::C.nth_half_step(-12), Note::C);
    }
}
