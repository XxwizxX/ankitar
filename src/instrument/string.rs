use crate::Note;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct GString {
    open_string: Note,
    fret_count: usize,
}

impl GString {
    pub fn new(open_string: Note, fret_count: usize) -> Self {
        GString {
            open_string,
            fret_count,
        }
    }

    pub fn visualize_selected_notes(&self, notes: &[Note]) {
        print!("{}|", self.open_string);
        for fret in 1..=self.fret_count {
            let note = self.open_string.nth_half_step(fret as i32);
            if notes.contains(&note) {
                print!("|--{}--", note);
            } else {
                print!("|------");
            }
        }
        println!("|");
    }
}

impl Display for GString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|", self.open_string)?;
        for fret in 1..=self.fret_count {
            let note = self.open_string.nth_half_step(fret as i32);
            write!(f, "|--{}--", note)?;
        }
        write!(f, "|")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g_string() {
        let g_string = GString::new(Note::E, 12);
        println!("{}", g_string);
    }
}
