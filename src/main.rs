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

fn scale(root: Note, intervals: [Interval; 7]) -> Vec<String> {
    let mut cycle_iterator = Note::iter().cycle();

    cycle_iterator.position( |note| note == root );

    // Sorry, but... we should rewrite below
    let notes = vec![root.to_s()];
    let total_intervals = intervals.len();
    let interval_list: &[Interval] = &intervals[..total_intervals-1];

    return interval_list.iter().fold(notes, |mut acc, &interval| {
        let current_note = cycle_iterator
            .nth(interval as usize - 1)
            .unwrap()
            .to_s();

        acc.push(current_note);
        acc
    });
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
