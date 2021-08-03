use super::{Note, NoteBuilder};

use nom::character::complete::{digit1, one_of};
use nom::combinator::{complete, eof, map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub struct NoteNom {
    //
}

impl NoteNom {
    pub fn parse(note: &str) -> Result<Note, String> {
        if note.is_empty() {
            return Err("Invalid note".to_string());
        }

        /*
        let mut chars = note.chars();
        let name = chars.next().unwrap();
        match name {
            'A'..='G' | 'a'..='g' => {
                builder.pitch(name.to_string().as_str());
            }
            _ => {
                return Err("Invalid note".to_string());
            }
        }

        if note.len() == 1 {
            return Ok(builder.build());
        }
        */

        fn name(input: &str) -> IResult<&str, char> {
            one_of("abcdefgABCDEFG")(input)
        }

        fn accidental(input: &str) -> IResult<&str, Option<&str>> {
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

        fn octave(input: &str) -> IResult<&str, i8> {
            map_res(
                recognize(tuple((opt(complete(one_of("+-"))), many1(digit1)))),
                |s| i8::from_str_radix(s, 10),
            )(input)
        }

        fn cents(input: &str) -> IResult<&str, i8> {
            map_res(
                recognize(tuple((opt(complete(one_of("+-"))), many1(digit1)))),
                |s| i8::from_str_radix(s, 10),
            )(input)
        }

        fn parse(input: &str) -> IResult<&str, (char, Option<&str>, Option<i8>, Option<i8>, &str)> {
            tuple((
                name,
                accidental,
                opt(complete(octave)),
                opt(complete(cents)),
                eof,
            ))(input)
        }

        let (_, (name, accidental, octave, cents, _)) =
            parse(note).map_err(|err| err.to_string())?;
        let mut builder = NoteBuilder::new();
        builder.name(note);
        builder.pitch(&name.to_string());
        if let Some(accidental) = accidental {
            builder.accidental(accidental);
        }
        if let Some(octave) = octave {
            builder.octave(octave);
        }
        if let Some(cents) = cents {
            builder.cents(cents);
        }
        Ok(builder.build())

        /*
        let (x, (accidental, octave, cents)) =
            parse(chars.as_str().as_bytes()).map_err(|err| err.to_string())?;
        println!("x after parse: {:?}", std::str::from_utf8(x).unwrap());
        println!("accidental: {:?}", accidental);
        println!("octave: {:?}", octave);
        println!("cents: {:?}", cents);
        if !accidental.is_empty() {
            println!("accidental: {:?}", accidental);
            builder.accidental(&accidental.into_iter().collect::<String>());
        }
        // if let Some(octave) = octave {
        //     builder.octave(unsafe { std::str::from_utf8_unchecked(octave) });
        // }
        if let Some((Some(cents), _)) = cents {
            builder.cents(&cents.to_string());
        }
        */

        // parse(input)
        // Err("xxx".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::NoteNom as Note;

    #[test]
    fn nom_parse_test() {
        // >>> librosa.note_to_midi('C')
        // 12
        // >>> librosa.note_to_midi('C#3')
        // 49
        // >>> librosa.note_to_midi('C♯3')  # Using Unicode sharp
        // 49
        // >>> librosa.note_to_midi('C♭3')  # Using Unicode flat
        // 47
        // >>> librosa.note_to_midi('f4')
        // 65
        // >>> librosa.note_to_midi('Bb-1')
        // 10
        // >>> librosa.note_to_midi('A!8')
        // 116
        // >>> librosa.note_to_midi('G𝄪6')  # Double-sharp
        // 93
        // >>> librosa.note_to_midi('B𝄫6')  # Double-flat
        // 93
        // >>> librosa.note_to_midi('C♭𝄫5')  # Triple-flats also work
        // 69
        // >>> # Lists of notes also work
        // >>> librosa.note_to_midi(['C', 'E', 'G'])
        // array([12, 16, 19])
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
