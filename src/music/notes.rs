use super::intervals::Interval;

use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Note {
    C4,
    Db4,
    D4,
    Eb4,
    E4,
    F4,
    Gb4,
    G4,
    Ab4,
    A4,
    Bb4,
    B4,
    C5,
}

impl Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Note::C4 => "C4",
            Note::Db4 => "Db4",
            Note::D4 => "D4",
            Note::Eb4 => "Eb4",
            Note::E4 => "E4",
            Note::F4 => "F4",
            Note::Gb4 => "Gb4",
            Note::G4 => "G4",
            Note::Ab4 => "Ab4",
            Note::A4 => "A4",
            Note::Bb4 => "Bb4",
            Note::B4 => "B4",
            Note::C5 => "C5",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "C4" => Ok(Note::C4),
            "Db4" => Ok(Note::Db4),
            "D4" => Ok(Note::D4),
            "Eb4" => Ok(Note::Eb4),
            "E4" => Ok(Note::E4),
            "F4" => Ok(Note::F4),
            "Gb4" => Ok(Note::Gb4),
            "G4" => Ok(Note::G4),
            "Ab4" => Ok(Note::Ab4),
            "A4" => Ok(Note::A4),
            "Bb4" => Ok(Note::Bb4),
            "B4" => Ok(Note::B4),
            "C5" => Ok(Note::C5),
            _ => Err(()),
        }
    }
}

impl Note {
    pub fn freqency(&self) -> f32 {
        match self {
            Note::C4 => 261.3,
            Note::Db4 => 277.18,
            Note::D4 => 293.66,
            Note::Eb4 => 313.13,
            Note::E4 => 329.63,
            Note::F4 => 349.23,
            Note::Gb4 => 369.99,
            Note::G4 => 392.0,
            Note::Ab4 => 414.30,
            Note::A4 => 440.0,
            Note::Bb4 => 466.16,
            Note::B4 => 493.88,
            Note::C5 => 523.25,
        }
    }

    pub fn position(&self) -> u8 {
        match self {
            Note::C4 => 0,
            Note::Db4 => 1,
            Note::D4 => 2,
            Note::Eb4 => 3,
            Note::E4 => 4,
            Note::F4 => 5,
            Note::Gb4 => 6,
            Note::G4 => 7,
            Note::Ab4 => 8,
            Note::A4 => 9,
            Note::Bb4 => 10,
            Note::B4 => 11,
            Note::C5 => 12,
        }
    }

    pub fn add_interval(&self, interval: &Interval) -> Note {
        (self.position() + interval.as_u8()).into()
    }
}

impl From<u8> for Note {
    fn from(value: u8) -> Self {
        match value {
            0 => Note::C4,
            1 => Note::Db4,
            2 => Note::D4,
            3 => Note::Eb4,
            4 => Note::E4,
            5 => Note::F4,
            6 => Note::Gb4,
            7 => Note::G4,
            8 => Note::Ab4,
            9 => Note::A4,
            10 => Note::Bb4,
            11 => Note::B4,
            12 => Note::C5,
            _ => Note::C5,
        }
    }
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        match self {
            Note::C4 => 0,
            Note::Db4 => 1,
            Note::D4 => 2,
            Note::Eb4 => 3,
            Note::E4 => 4,
            Note::F4 => 5,
            Note::Gb4 => 6,
            Note::G4 => 7,
            Note::Ab4 => 8,
            Note::A4 => 9,
            Note::Bb4 => 10,
            Note::B4 => 11,
            Note::C5 => 12,
        }
    }
}

// impl Add<Interval> for Note {
//     type Output = Self;
//
//     fn add(self, interval: Interval) -> Self::Output {
//         let note_position: u8 = self.into();
//         let interval: u8 = interval.into();
//         let new_note: u8 = note_position + interval;
//         new_note.into()
//     }
// }