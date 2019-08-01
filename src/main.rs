extern crate strum;
#[macro_use] extern crate strum_macros;

use strum::IntoEnumIterator;
use std::fmt;

#[derive(Copy, Clone, Debug)]
enum Interval {
    Half = 1,
    Whole = 2
}

#[derive(Copy, Clone, EnumIter, PartialEq)]
enum Note {
    C, Cs, D, Ds, E, F,
    Fs, G, Gs, A, As, B
}

impl fmt::Display for Note {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Note::C => write!(fmt, "C"),
            Note::Cs => write!(fmt, "C#"),
            Note::D => write!(fmt, "D"),
            Note::Ds => write!(fmt, "D#"),
            Note::E => write!(fmt, "E"),
            Note::F => write!(fmt, "F"),
            Note::Fs => write!(fmt, "F#"),
            Note::G => write!(fmt, "G"),
            Note::Gs => write!(fmt, "G#"),
            Note::A => write!(fmt, "A"),
            Note::As => write!(fmt, "A#"),
            Note::B => write!(fmt, "B"),
        }
    }
}

fn scale(root: Note, intervals: Vec<Interval>) -> Vec<String> {
    let mut cloned_intervals = intervals.clone();
    cloned_intervals.pop();

    let mut cycle_iterator = Note::iter().cycle();
    let notes = vec![root];

    cycle_iterator.position( |note| note == root );

    return cloned_intervals.iter().fold(notes, |mut acc, &interval| {
        let current_note = cycle_iterator
            .nth(interval as usize - 1)
            .unwrap();

        acc.push(current_note);
        acc
    }).iter().map(|note| note.to_string()).collect();
}

#[allow(dead_code)]
fn major_scale(root: Note) -> Vec<String> {
    return scale(root, [
        Interval::Whole,
        Interval::Whole,
        Interval::Half,
        Interval::Whole,
        Interval::Whole,
        Interval::Whole,
        Interval::Half,
    ].to_vec());
}

#[allow(dead_code)]
fn minor_scale(root: Note) -> Vec<String> {
    return scale(root, [
        Interval::Whole,
        Interval::Half,
        Interval::Whole,
        Interval::Whole,
        Interval::Half,
        Interval::Whole,
        Interval::Whole,
    ].to_vec());
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intervals() {
        assert_eq!(Interval::Half as usize, 1);
        assert_eq!(Interval::Whole as usize, 2);
    }

    #[test]
    fn test_notes() {
        assert!(Note::iter().eq([
            Note::C, Note::Cs, Note::D, Note::Ds,
            Note::E, Note::F, Note::Fs, Note::G,
            Note::Gs, Note::A, Note::As, Note::B
        ].iter().cloned()));
    }

    #[test]
    fn test_c_major_scale() {
        assert_eq!(
            major_scale(Note::C),
            ["C", "D", "E", "F", "G", "A", "B"]
        );
    }

    #[test]
    fn test_b_minor_scale() {
        assert_eq!(
            minor_scale(Note::B),
            ["B", "C#", "D", "E", "F#", "G", "A"]
        );
    }
}
