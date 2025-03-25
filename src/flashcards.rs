use color_eyre::Result;
use std::{ffi::OsStr, fs::read_dir, io, path::PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FlashcardSet {
    pub(crate) name: String,
    pub(crate) cards: Vec<Flashcard>,
    pub(crate) current_card_idx: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Flashcard {
    pub(crate) front: String,
    pub(crate) back: String,
    pub(crate) current_side: CurrentSide,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CurrentSide {
    Front,
    Back,
}

impl FlashcardSet {
    pub(crate) fn next_card(&mut self) -> Option<&Flashcard> {
        if self.current_card_idx <= self.cards.len() - 1 {
            self.current_card_idx += 1;
            return self.cards.get(self.current_card_idx);
        }
        return None;
    }

    pub(crate) fn prev_card(&mut self) -> Option<&Flashcard> {
        if self.current_card_idx > 0 {
            self.current_card_idx -= 1;
        }
        self.cards.get(self.current_card_idx)
    }
}

impl Flashcard {
    pub(crate) fn current_side(&self) -> String {
        match self.current_side {
            CurrentSide::Back => self.back.clone(),
            CurrentSide::Front => self.front.clone(),
        }
    }

    pub(crate) fn flip(&mut self) {
        match self.current_side {
            CurrentSide::Back => self.current_side = CurrentSide::Front,
            CurrentSide::Front => self.current_side = CurrentSide::Back,
        }
    }

    pub(crate) fn calc_vert_size(&self) -> u16 {
        // FIXME: so many bad things here
        if self.front.len() >= self.back.len() {
            return std::cmp::max(11, (10 + self.front.lines().count()).try_into().unwrap());
        }
        return std::cmp::max(11, (10 + self.back.lines().count()).try_into().unwrap());
    }
}

// MARK: Parsing Funcs

/// Returns the name of and path to all csv files in a given dir
pub(crate) fn find_all_sets(fp: String) -> Result<Vec<(String, PathBuf)>, io::Error> {
    Ok(read_dir(fp)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.extension() == Some(OsStr::new("csv")))
        .map(|r| (r.file_stem().unwrap().to_str().unwrap().to_string(), r))
        .collect())
}

/// Returns a FlashcardSet from a given file path and name
pub(crate) fn parse_set(csv_string: &str, name: String, capitalize: bool, reverse: bool) -> FlashcardSet {
    let mut read = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .quoting(true)
        .from_reader(csv_string.trim().as_bytes());
    let cards = read
        .records()
        .map(|card| {
            // FIXME: so many bad things here
            let mut front = card.as_ref().unwrap().get(0).unwrap().to_string();
            let mut back = card.unwrap().get(1).unwrap().to_string();

            if capitalize {
                front = front.to_uppercase();
                back = back.to_uppercase();
            }

            Flashcard {
                front,
                back,
                current_side: if reverse { CurrentSide::Back } else { CurrentSide::Front },
            }
        })
        .collect();
    return FlashcardSet {
        name,
        cards,
        current_card_idx: 0,
    };
}

#[test]
fn parse_simple_set() {
    let set_csv = "
    foo,bar
    baz,thenextone
    ";

    let set = parse_set(set_csv, "test".to_string(), false, false);
    assert_eq!(
        set,
        FlashcardSet {
            name: "test".to_string(),
            cards: vec![
                Flashcard {
                    front: "foo".to_string(),
                    back: "bar".to_string(),
                    current_side: CurrentSide::Front,
                },
                Flashcard {
                    front: "baz".to_string(),
                    back: "thenextone".to_string(),
                    current_side: CurrentSide::Front,
                },
            ],
            current_card_idx: 0,
        }
    );
}

#[test]
fn parse_newline_literals() {
    let set_csv = "\"line1\nline2\",baz";

    let set = parse_set(set_csv, "test".to_string(), false, false);
    assert_eq!(
        set,
        FlashcardSet {
            name: "test".to_string(),
            cards: vec![Flashcard {
                front: "line1\nline2".to_string(),
                back: "baz".to_string(),
                current_side: CurrentSide::Front,
            },],
            current_card_idx: 0,
        }
    );
}
