mod note;
mod scale;

use clap::Parser;
pub use note::Note;
pub use scale::*;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;

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
    println!("{:?}", result)
}

// TODO change to linked list
struct Notes(Graph<Note, ()>);

fn get_node_index_by_value<N: PartialEq>(
    graph: &Graph<N, ()>,
    target_value: &N,
) -> Option<NodeIndex> {
    graph.node_indices().find(|&node_idx| {
        graph
            .node_weight(node_idx)
            .map_or(false, |weight| weight == target_value)
    })
}

impl Notes {
    fn init() -> Self {
        let mut all_notes = Graph::<Note, ()>::new();

        let c = all_notes.add_node(Note::C);
        let c_sharp = all_notes.add_node(Note::Db);
        let d = all_notes.add_node(Note::D);
        let d_sharp = all_notes.add_node(Note::Eb);
        let e = all_notes.add_node(Note::E);
        let f = all_notes.add_node(Note::F);
        let f_sharp = all_notes.add_node(Note::Gb);
        let g = all_notes.add_node(Note::G);
        let g_sharp = all_notes.add_node(Note::Ab);
        let a = all_notes.add_node(Note::A);
        let a_sharp = all_notes.add_node(Note::Bb);
        let b = all_notes.add_node(Note::B);

        all_notes.add_edge(c, c_sharp, ());
        all_notes.add_edge(c_sharp, d, ());
        all_notes.add_edge(d, d_sharp, ());
        all_notes.add_edge(d_sharp, e, ());
        all_notes.add_edge(e, f, ());
        all_notes.add_edge(f, f_sharp, ());
        all_notes.add_edge(f_sharp, g, ());
        all_notes.add_edge(g, g_sharp, ());
        all_notes.add_edge(g_sharp, a, ());
        all_notes.add_edge(a, a_sharp, ());
        all_notes.add_edge(a_sharp, b, ());
        all_notes.add_edge(b, c, ());

        Notes(all_notes)
    }

    fn half_step(&self, note: &Note) -> Note {
        let root = get_node_index_by_value(&self.0, note).unwrap();
        let mut neighbors = self.0.neighbors_directed(root, petgraph::Outgoing);
        let neighbor = neighbors.next().unwrap();
        self.0[neighbor].clone()
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
