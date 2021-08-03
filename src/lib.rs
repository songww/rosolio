use std::collections::HashMap;

pub mod by_nom;
pub mod by_pest;
pub mod by_regex;

#[macro_use]
extern crate pest_derive;

use once_cell::sync::Lazy;

static PITCH_MAP: Lazy<HashMap<&str, i8>> = Lazy::new(|| {
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

static ACC_MAP: Lazy<HashMap<char, i8>> = Lazy::new(|| {
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

#[derive(Debug, Clone)]
pub struct Note {
    name: String,
    pitch: u8,
    _pitch: i8,
    accidental: i8,
    octave: i8,
    cents: f32,
}

impl Note {
    fn new(name: String, pitch: i8, accidental: i8, octave: i8, cents: f32) -> Self {
        let _pitch = pitch;
        let pitch = 12 * (octave + 1) + _pitch + accidental;
        // println!(
        //     "pitch({}) = 12 * (octave:{} + 1) + _pitch:{} + accidental:{}",
        //     pitch, octave, _pitch, accidental
        // );

        // let pitch +=  cents;

        Note {
            pitch: pitch as u8,
            _pitch,
            name,
            octave,
            accidental,
            cents,
        }
    }

    pub fn pitch(&self) -> u8 {
        self.pitch
    }
}

struct NoteBuilder {
    name: String,
    pitch: i8,
    accidental: Option<i8>,
    octave: Option<i8>,
    cents: Option<f32>,
}

impl NoteBuilder {
    fn new() -> Self {
        Self {
            name: "C".to_string(),
            pitch: 0,
            accidental: None,
            octave: None,
            cents: None,
        }
    }

    fn name<S: ToString>(&mut self, name: S) -> &mut Self {
        self.name = name.to_string();
        self
    }

    fn pitch(&mut self, pitch: &str) -> &mut Self {
        let pitch = PITCH_MAP.get(pitch).unwrap();
        self.pitch = *pitch;
        self
    }

    fn accidental(&mut self, accidental: &str) -> &mut Self {
        let accidental = accidental.chars().map(|o| ACC_MAP.get(&o).unwrap()).sum();
        // println!("accidental: {}", accidental);
        self.accidental.replace(accidental);
        self
    }

    fn octave(&mut self, octave: i8) -> &mut Self {
        self.octave.replace(octave);
        self
    }

    fn cents(&mut self, cents: i8) -> &mut Self {
        self.cents.replace(cents as f32 * 1e-2_f32);
        self
    }

    fn build(self) -> Note {
        Note::new(
            self.name,
            self.pitch,
            self.accidental.unwrap_or(0),
            self.octave.unwrap_or(0),
            self.cents.unwrap_or(0.),
        )
    }
}
