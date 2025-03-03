use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{read_dir, read_to_string},
    io,
    path::PathBuf,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

/// Returns the name of and path to all csv files in a given dir
fn find_all_sets(fp: String) -> Result<Vec<(String, PathBuf)>, io::Error> {
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
fn parse_set(csv_string: &str, name: String) -> FlashcardSet {
    let cards = csv_string
        .trim()
        .split('\n')
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| Flashcard {
                    side_a: a.trim().to_string(),
                    side_b: b.trim().to_string(),
                })
                .unwrap()
        })
        .collect();
    return FlashcardSet { name, cards };
}

#[derive(Debug)]
struct OneRunStats {
    correct: HashSet<Flashcard>,
    incorrect: HashSet<Flashcard>,
}

#[derive(Debug, PartialEq)]
struct FlashcardSet {
    name: String,
    cards: Vec<Flashcard>,
}

#[derive(Debug, PartialEq)]
struct Flashcard {
    side_a: String,
    side_b: String,
}

#[test]
fn parse_simple_set() {
    let set_csv = "
    foo,bar
    baz,thenextone
    ";

    let set = parse_set(set_csv, "test".to_string());
    assert_eq!(
        set,
        FlashcardSet {
            name: "test".to_string(),
            cards: vec![
                Flashcard {
                    side_a: "foo".to_string(),
                    side_b: "bar".to_string()
                },
                Flashcard {
                    side_a: "baz".to_string(),
                    side_b: "thenextone".to_string()
                }
            ]
        }
    );
}
