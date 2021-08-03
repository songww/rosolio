use std::collections::HashMap;
use std::str::FromStr;

use nom::character::complete::{digit1, one_of};
use nom::combinator::{complete, map_res, opt, recognize};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{Finish, IResult};
use once_cell::sync::Lazy;
use thiserror::Error;

use crate::core::note::Note;

static PITCH_NAMES: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(7);
    map.insert('C', "C");
    map.insert('D', "D");
    map.insert('E', "E");
    map.insert('F', "F");
    map.insert('G', "G");
    map.insert('A', "A");
    map.insert('B', "B");
    map
});

pub(crate) static NAME_TO_PITCH: Lazy<HashMap<&str, i8>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("C", 0);
    map.insert("D", 2);
    map.insert("E", 4);
    map.insert("F", 5);
    map.insert("G", 7);
    map.insert("A", 9);
    map.insert("B", 11);
    map.insert("c", 0);
    map.insert("d", 2);
    map.insert("e", 4);
    map.insert("f", 5);
    map.insert("g", 7);
    map.insert("a", 9);
    map.insert("b", 11);
    map
});

pub(crate) static ACCIDENTAL_TO_I8: Lazy<HashMap<char, i8>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('#', 1);
    map.insert('\x00', 0);
    map.insert('b', -1);
    map.insert('!', -1);
    map.insert('♯', 1);
    map.insert('𝄪', 2);
    map.insert('♭', -1);
    map.insert('𝄫', -2);
    map.insert('♮', 0);
    map
});

pub(crate) static I8_TO_ACCIDENTAL: Lazy<HashMap<i8, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0, "");
    map.insert(1, "♯");
    map.insert(2, "𝄪");
    map.insert(3, "♯𝄪");
    map.insert(4, "𝄪𝄪");
    map.insert(-1, "♭");
    map.insert(-2, "𝄫");
    map.insert(-3, "♭𝄫");
    map.insert(-4, "𝄫𝄫");
    map
});

#[derive(Debug, Error)]
pub enum NoteParseError {
    #[error("{0}")]
    ParseError(String),
}

impl FromStr for Note {
    type Err = NoteParseError;
    fn from_str(input: &str) -> Result<Note, NoteParseError> {
        fn pitchname_(input: &str) -> IResult<&str, char> {
            one_of("abcdefgABCDEFG")(input)
        }

        fn accidental_(input: &str) -> IResult<&str, Option<&str>> {
            map_res(
                recognize(opt(many1(one_of("#♯𝄪b!♭𝄫♮")))),
                |s: &str| -> Result<Option<&str>, ()> {
                    if s.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(s))
                    }
                },
            )(input)
        }

        fn octave_(input: &str) -> IResult<&str, i8> {
            map_res(
                recognize(tuple((opt(complete(one_of("+-"))), many1(digit1)))),
                |s| i8::from_str_radix(s, 10),
            )(input)
        }

        fn cents_(input: &str) -> IResult<&str, i8> {
            map_res(
                recognize(tuple((opt(complete(one_of("+-"))), many1(digit1)))),
                |s| i8::from_str_radix(s, 10),
            )(input)
        }

        fn parse(input: &str) -> IResult<&str, (char, Option<&str>, Option<i8>, Option<i8>)> {
            tuple((
                pitchname_,
                accidental_,
                opt(complete(octave_)),
                opt(complete(cents_)),
            ))(input)
        }

        let (_, (pitchname, accidental, octave, cents)) = parse(input)
            .finish()
            .map_err(|err| NoteParseError::ParseError(err.to_string()))?;

        let pitchname = PITCH_NAMES.get(&pitchname.to_ascii_uppercase()).unwrap();
        // .expect(&format!(
        //         "Invalid pitchname `{}`, should be one of `{:?}`",
        //         pitchname,
        //         PITCH_NAMES.values().collect::<Vec<_>>().as_slice()
        // ))
        let accidental = accidental.unwrap_or("");
        let octave = octave.unwrap_or(0);
        let cents = cents.unwrap_or(0);
        let accidental = accidental
            .chars()
            .map(|o| ACCIDENTAL_TO_I8.get(&o).unwrap())
            .sum();
        Ok(Note::new(pitchname, accidental, octave + 1, cents))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::note::Note;

    #[test]
    fn nom_parse_test() {
        let v: Note = "C".parse().unwrap();
        assert_eq!(v.pitch(), 12, "C parsed to: {:?}", v);
        let v: Note = "C#3".parse().unwrap();
        assert_eq!(v.pitch(), 49, "C#3 parsed to: {:?}", v);
        let v: Note = "C♯3".parse().unwrap(); // Using Unicode sharp
        assert_eq!(v.pitch(), 49, "C♯3 parsed to: {:?}", v);
        let v: Note = "C♭3".parse().unwrap(); // Using Unicode flat
        assert_eq!(v.pitch(), 47, "C♭3 parsed to: {:?}", v);
        let v: Note = "f4".parse().unwrap();
        assert_eq!(v.pitch(), 65, "f4 parsed to: {:?}", v);
        let v: Note = "Bb-1".parse().unwrap();
        assert_eq!(v.pitch(), 10, "Bb-1 parsed to: {:?}", v);
        let v: Note = "A!8".parse().unwrap();
        assert_eq!(v.pitch(), 116, "A!8 parsed to: {:?}", v);
        let v: Note = "G𝄪6".parse().unwrap(); // Double-sharp
        assert_eq!(v.pitch(), 93, "G𝄪6 parsed to: {:?}", v);
        let v: Note = "B𝄫6".parse().unwrap(); // Double-flat
        assert_eq!(v.pitch(), 93, "B𝄫6 parsed to: {:?}", v);
        let v: Note = "C♭𝄫5".parse().unwrap(); // Triple-flats also work
        assert_eq!(v.pitch(), 69, "C♭𝄫5 parsed to: {:?}", v);
        let v = "Z♭𝄫5".parse::<Note>().unwrap_err();
        assert_eq!("error OneOf at: Z♭𝄫5", &v.to_string());
        let v = "".parse::<Note>().unwrap_err();
        assert_eq!("error OneOf at: ", &v.to_string());
    }
}
