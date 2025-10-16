use crate::Note;
pub fn format_notes(notes: &Vec<Note>) -> Vec<String> {
    let mut result = Vec::new();
    for note in notes {
        match note {
            Note::Db => {
                if notes.contains(&Note::D) {
                    result.push("C#".to_string())
                } else {
                    result.push("Db".to_string())
                }
            }
            Note::Eb => {
                if notes.contains(&Note::E) {
                    result.push("D#".to_string())
                } else {
                    result.push("Eb".to_string())
                }
            }
            Note::Gb => {
                if notes.contains(&Note::G) {
                    result.push("F#".to_string())
                } else {
                    result.push("Gb".to_string())
                }
            }
            Note::Ab => {
                if notes.contains(&Note::A) {
                    result.push("G#".to_string())
                } else {
                    result.push("Ab".to_string())
                }
            }
            Note::Bb => {
                if notes.contains(&Note::B) {
                    result.push("A#".to_string())
                } else {
                    result.push("Bb".to_string())
                }
            }
            _ => result.push(note.to_string()),
        }
    }
    result
}
