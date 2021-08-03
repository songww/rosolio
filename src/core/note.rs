use super::converters::note_converter;

#[derive(Debug, Clone)]
pub struct Note {
    pitchname: &'static str,
    pitch: i8,
    accidental: i8,
    octave: i8,
    cents: i8,
}

impl Note {
    pub fn new(pitchname: &'static str, accidental: i8, octave: i8, cents: i8) -> Self {
        let pitch: i8 = *note_converter::NAME_TO_PITCH.get(pitchname).unwrap();
        Note {
            pitchname,
            pitch,
            octave,
            accidental,
            cents,
        }
    }

    pub fn pitch(&self) -> u8 {
        let cents = (self.cents as f32 * 1e-2_f32).round() as u8;
        (12 * self.octave + self.pitch + self.accidental) as u8 + cents
    }

    pub fn pitch_f32(&self) -> f32 {
        let cents = self.cents as f32 * 1e-2_f32;
        (12 * self.octave + self.pitch + self.accidental) as f32 + cents
    }

    pub fn octave(&self) -> u8 {
        self.octave as u8
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pitchname)?;
        f.write_fmt(format_args!(
            "{}",
            note_converter::I8_TO_ACCIDENTAL
                .get(&self.accidental)
                .unwrap(),
        ))?;
        if self.octave > 1 {
            f.write_fmt(format_args!("{}", self.octave))?;
        }
        if self.cents > 0 {
            f.write_fmt(format_args!("+{}", self.cents))?
        } else if self.cents < 0 {
            f.write_fmt(format_args!("-{}", self.cents))?
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
mod serde_ {
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    use super::Note;

    impl Serialize for Note {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for Note {
        fn deserialize<D>(deserializer: D) -> Result<Note, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(de::Error::custom)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::Note;

        #[test]
        fn test_serialize() {
            assert_eq!(
                serde_json::to_string(&Note::new("C", 1, 3, 0)).unwrap(),
                "\"C♯3\"".to_string()
            );
        }

        #[test]
        fn test_deserialize() {
            // Note::new("C", 1, 3, 0)
            let note: Note = serde_json::from_str("\"C♯3\"").unwrap();
            assert_eq!(note.pitch(), 49);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Note;

    #[test]
    fn test_display() {
        let note = Note::new("C", 0, 0, 0);
        assert_eq!(note.to_string(), "C".to_string());
        assert_eq!(note.pitch(), 0);
        let note = Note::new("C", 0, 1, 0);
        assert_eq!(note.to_string(), "C".to_string());
        assert_eq!(note.pitch(), 12);
        assert_eq!(Note::new("C", 1, 3, 0).to_string(), "C♯3".to_string());
    }
}
