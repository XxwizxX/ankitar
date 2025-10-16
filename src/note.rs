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

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}