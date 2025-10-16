mod note;
mod scale;
mod fmt;

use clap::Parser;
pub use note::Note;
pub use scale::*;
pub use fmt::format_notes;
use std::collections::LinkedList;

#[derive(Parser)]
struct Args {
    /// root note
    #[arg(short, long, value_enum)]
    root: Note,

    /// scale to use
    #[arg(short, long, value_enum)]
    scale: ScaleType,
}

fn main() {
    let args = Args::parse();
    let notes = Notes::init();

    let root = args.root;
    let scale = Scale::from_type(args.scale);

    let result = notes.get_notes(&root, &scale);
    let result = format_notes(&result);
    println!("{:?}", result)
}

struct Notes(LinkedList<Note>);

fn find_index<T: PartialEq>(list: &LinkedList<T>, value: &T) -> Option<usize> {
    for (index, element) in list.iter().enumerate() {
        if element == value {
            return Some(index);
        }
    }
    None
}

impl Notes {
    fn init() -> Self {
        let mut list: LinkedList<Note> = LinkedList::new();
        list.push_back(Note::C);
        list.push_back(Note::Db);
        list.push_back(Note::D);
        list.push_back(Note::Eb);
        list.push_back(Note::E);
        list.push_back(Note::F);
        list.push_back(Note::Gb);
        list.push_back(Note::G);
        list.push_back(Note::Ab);
        list.push_back(Note::A);
        list.push_back(Note::Bb);
        list.push_back(Note::B);

        Notes(list)
    }

    fn half_step(&self, note: &Note) -> Note {
        let idx = find_index(&self.0, note).unwrap();
        self.0.iter().nth((idx + 1) % self.0.len()).unwrap().clone()
    }

    fn whole_step(&self, note: &Note) -> Note {
        self.half_step(&self.half_step(note)).clone()
    }

    fn minor_third(&self, note: &Note) -> Note {
        self.whole_step(&self.half_step(note)).clone()
    }

    fn get_notes(&self, root: &Note, scale: &Scale) -> Vec<Note> {
        let mut notes = vec![root.clone()];
        for interval in &scale.0 {
            let next_note = match interval {
                Interval::HalfStep => self.half_step(notes.last().unwrap()),
                Interval::WholeStep => self.whole_step(notes.last().unwrap()),
                Interval::MinorThird => self.minor_third(notes.last().unwrap()),
            };
            notes.push(next_note);
        }
        notes
    }
}
