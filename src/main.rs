extern crate strum;
#[macro_use] extern crate strum_macros;

use strum::IntoEnumIterator;

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum Interval {
    Half = 1,
    Whole = 2
}

#[derive(Clone, EnumIter, PartialEq)]
enum Note {
    C, Cs, D, Ds, E, F,
    Fs, G, Gs, A, As, B
}

impl Note {
    fn to_s(&self) -> String {
        match *self {
            Note::C => "C".to_string(),
            Note::Cs => "C#".to_string(),
            Note::D => "D".to_string(),
            Note::Ds => "D#".to_string(),
            Note::E => "E".to_string(),
            Note::F => "F".to_string(),
            Note::Fs => "F#".to_string(),
            Note::G => "G".to_string(),
            Note::Gs => "G#".to_string(),
            Note::A => "A".to_string(),
            Note::As => "A#".to_string(),
            Note::B => "B".to_string(),
        }
    }
}

#[allow(dead_code)]
fn scale(root: Note, intervals: [Interval; 7]) -> Vec<String> {
    let mut cycle_iterator = Note::iter().cycle();

    cycle_iterator.position( |note| note == root );

    intervals.iter().map( |&interval| {
        let interval_value = interval as usize;
        cycle_iterator
            .nth(interval_value - 1)
            .unwrap()
            .to_s()
    }).collect()
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
    ]);
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
    ]);
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
            ["D", "E", "F", "G", "A", "B", "C"]
        );
    }

    #[test]
    fn test_b_minor_scale() {
        assert_eq!(
            minor_scale(Note::B),
            ["C#", "D", "E", "F#", "G", "A", "B"]
        );
    }
}
