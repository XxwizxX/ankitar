use clap::ValueEnum;
use crate::Note;

#[derive(Debug)]
pub enum Interval {
    HalfStep,
    WholeStep,
    MinorThird,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ScaleType {
    Major,
    Minor,
    MajorPentatonic,
    MinorPentatonic,
}

#[derive(Debug)]
pub struct Scale(pub Vec<Interval>);

impl Scale {
    pub fn from_type(scale_type: ScaleType) -> Self {
        match scale_type {
            ScaleType::Major => Scale::major(),
            ScaleType::Minor => Scale::minor(),
            ScaleType::MajorPentatonic => Scale::major_pentatonic(),
            ScaleType::MinorPentatonic => Scale::minor_pentatonic(),
        }
    }

    pub fn root(&self, note: &Note) -> Vec<Note> {
        let mut result: Vec<Note> = Vec::new();
        result.push(note.clone());
        self.0.iter().for_each(|interval| {
            let last_note = result.last().unwrap();
            let next_note = match interval {
                Interval::HalfStep => last_note.nth_half_step(1),
                Interval::WholeStep => last_note.nth_half_step(2),
                Interval::MinorThird => last_note.nth_half_step(3),
            };
            result.push(next_note);
        });
        result
    }

    fn major() -> Self {
        let intervals = vec![
            Interval::WholeStep,
            Interval::WholeStep,
            Interval::HalfStep,
            Interval::WholeStep,
            Interval::WholeStep,
            Interval::WholeStep,
            Interval::HalfStep,
        ];
        Scale(intervals)
    }

    fn minor() -> Self {
        let intervals = vec![
            Interval::WholeStep,
            Interval::HalfStep,
            Interval::WholeStep,
            Interval::WholeStep,
            Interval::HalfStep,
            Interval::WholeStep,
            Interval::WholeStep,
        ];
        Scale(intervals)
    }

    fn major_pentatonic() -> Self {
        let intervals = vec![
            Interval::WholeStep,
            Interval::WholeStep,
            Interval::MinorThird,
            Interval::WholeStep,
            Interval::MinorThird,
        ];
        Scale(intervals)
    }

    fn minor_pentatonic() -> Self {
        let intervals = vec![
            Interval::MinorThird,
            Interval::WholeStep,
            Interval::WholeStep,
            Interval::MinorThird,
            Interval::WholeStep,
        ];
        Scale(intervals)
    }
}
