extern crate pest;

use pest::Parser;

use super::{Note, NoteBuilder};

#[derive(Parser)]
#[grammar = "note.pest"]
pub struct NoteParser;

pub struct NotePest {}

impl NotePest {
    pub fn parse(note: &str) -> Result<Note, String> {
        /*
        let tokens = pest::state::<(), _>(note, |s| {
            println!("s: {:?}", s);
            Ok(s)
        })
        .unwrap();
        */
        let mut tokens = NoteParser::parse(Rule::note, note)
            .map_err(|_| format!("Improper note format: {}", note))?;
        let mut note_builder = NoteBuilder::new();
        note_builder.name(note);
        for pair in tokens
            .next()
            .ok_or(format!("Improper note format: {}", note))?
            .into_inner()
        {
            match pair.as_rule() {
                Rule::name => {
                    note_builder.pitch(pair.as_str());
                }
                Rule::accidental => {
                    note_builder.accidental(pair.as_str());
                }
                Rule::octave => {
                    note_builder.octave(pair.as_str().parse().unwrap_or(0));
                }
                Rule::cents => {
                    note_builder.cents(pair.as_str().parse().unwrap_or(0));
                }
                _ => {
                    //println!(
                    //    "unexpected pair rule: {:?}, value: {:?}",
                    //    pair.as_rule(),
                    //    pair.as_str()
                    //);
                }
            }
            /*
            println!(
                "pair rule: {:?}, value: {:?}",
                pair.as_rule(),
                pair.as_str()
            );
            */
        }
        Ok(note_builder.build())
    }
}

#[cfg(test)]
mod tests {
    use super::NotePest as Note;

    #[test]
    fn pest_parse_test() {
        /// >>> librosa.note_to_midi('C')
        /// 12
        /// >>> librosa.note_to_midi('C#3')
        /// 49
        /// >>> librosa.note_to_midi('C♯3')  # Using Unicode sharp
        /// 49
        /// >>> librosa.note_to_midi('C♭3')  # Using Unicode flat
        /// 47
        /// >>> librosa.note_to_midi('f4')
        /// 65
        /// >>> librosa.note_to_midi('Bb-1')
        /// 10
        /// >>> librosa.note_to_midi('A!8')
        /// 116
        /// >>> librosa.note_to_midi('G𝄪6')  # Double-sharp
        /// 93
        /// >>> librosa.note_to_midi('B𝄫6')  # Double-flat
        /// 93
        /// >>> librosa.note_to_midi('C♭𝄫5')  # Triple-flats also work
        /// 69
        /// >>> # Lists of notes also work
        /// >>> librosa.note_to_midi(['C', 'E', 'G'])
        /// array([12, 16, 19])
        let v = Note::parse("C").unwrap();
        assert_eq!(v.pitch(), 12, "C parsed to: {:?}", v);
        let v = Note::parse("C#3").unwrap();
        assert_eq!(v.pitch(), 49, "C#3 parsed to: {:?}", v);
        let v = Note::parse("f4").unwrap();
        assert_eq!(v.pitch(), 65, "f4 parsed to: {:?}", v);
        let v = Note::parse("Bb-1").unwrap();
        assert_eq!(v.pitch(), 10, "Bb-1 parsed to: {:?}", v);
        let v = Note::parse("A!8").unwrap();
        assert_eq!(v.pitch(), 116, "A!8 parsed to: {:?}", v);
        let v = Note::parse("G𝄪6").unwrap();
        assert_eq!(v.pitch(), 93, "G𝄪6 parsed to: {:?}", v);
        let v = Note::parse("B𝄫6").unwrap();
        assert_eq!(v.pitch(), 93, "B𝄫6 parsed to: {:?}", v);
        let v = Note::parse("C♭𝄫5").unwrap();
        assert_eq!(v.pitch(), 69, "C♭𝄫5 parsed to: {:?}", v);
    }
}
