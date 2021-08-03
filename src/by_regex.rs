use once_cell::sync::Lazy;
use regex::Regex;

use super::{Note, NoteBuilder};

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?P<name>[A-Ga-g])(?P<accidental>[#♯𝄪b!♭𝄫♮]*)(?P<octave>[+-]?\d+)?(?P<cents>[+-]\d+)?$",
    )
    .unwrap()
});

pub struct NoteRegex {}

impl NoteRegex {
    pub fn parse(note: &str) -> Result<Note, String> {
        let n = REGEX
            .captures(note)
            .ok_or(format!("Improper note format: {}", note))?;
        // raise ParameterError("Improper note format: {:s}".format(note))
        let mut note_builder = NoteBuilder::new();

        let pitch = n.name("name").unwrap();
        let accidental = n.name("accidental").unwrap();

        note_builder.accidental(accidental.as_str());

        n.name("octave")
            .map(|octave| note_builder.octave(octave.as_str().parse().unwrap_or(0)));
        n.name("cents")
            .map(|cents| note_builder.cents(cents.as_str().parse().unwrap_or(0)));

        note_builder.pitch(pitch.as_str());

        /*
        Ok(Note::new(
            /* name: */ note.to_string(),
            // pitch: pitch as u8,
            /* pitch: */ *pitch,
            octave,
            offset,
            cents,
        ))
        */
        Ok(note_builder.build())
    }
}

#[cfg(test)]
mod tests {
    use super::NoteRegex;

    #[test]
    fn regex_parse_test() {
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
        let v = NoteRegex::parse("C").unwrap();
        assert_eq!(v.pitch(), 12, "C parsed to: {:?}", v);
        let v = NoteRegex::parse("C#3").unwrap();
        assert_eq!(v.pitch(), 49, "C#3 parsed to: {:?}", v);
        let v = NoteRegex::parse("f4").unwrap();
        assert_eq!(v.pitch(), 65, "f4 parsed to: {:?}", v);
        let v = NoteRegex::parse("Bb-1").unwrap();
        assert_eq!(v.pitch(), 10, "Bb-1 parsed to: {:?}", v);
        let v = NoteRegex::parse("A!8").unwrap();
        assert_eq!(v.pitch(), 116, "A!8 parsed to: {:?}", v);
        let v = NoteRegex::parse("G𝄪6").unwrap();
        assert_eq!(v.pitch(), 93, "G𝄪6 parsed to: {:?}", v);
        let v = NoteRegex::parse("B𝄫6").unwrap();
        assert_eq!(v.pitch(), 93, "B𝄫6 parsed to: {:?}", v);
        let v = NoteRegex::parse("C♭𝄫5").unwrap();
        assert_eq!(v.pitch(), 69, "C♭𝄫5 parsed to: {:?}", v);
    }
}
