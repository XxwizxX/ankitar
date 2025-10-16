use clap::ValueEnum;

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
