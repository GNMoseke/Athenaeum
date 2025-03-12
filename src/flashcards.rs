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

// TODO: handle these:
// - set metadata in comments
// - double quoted raw strings

/// Returns a FlashcardSet from a given file path and name
pub(crate) fn parse_set(csv_string: &str, name: String, capitalize: bool) -> FlashcardSet {
    let cards = csv_string
        .trim()
        .split('\n')
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| Flashcard {
                    front: if capitalize {
                        a.trim().to_uppercase().to_string()
                    } else {
                        a.trim().to_string()
                    },
                    back: if capitalize {
                        b.trim().to_uppercase().to_string()
                    } else {
                        b.trim().to_string()
                    },
                    current_side: CurrentSide::Front,
                })
                .unwrap()
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

    let set = parse_set(set_csv, "test".to_string(), false);
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
