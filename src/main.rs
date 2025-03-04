use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{collections::HashSet, ffi::OsStr, fs::read_dir, io, path::PathBuf};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_res = App { exit: false }.run(&mut terminal);
    ratatui::restore();
    app_res
}

struct App {
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame<'_>) {
        frame.render_widget(
            &FlashcardSet {
                name: "Test".to_string(),
                cards: vec![Flashcard {
                    side_a: "A".to_string(),
                    side_b: "B".to_string(),
                }],
            },
            frame.area(),
        )
    }

    fn exit(&mut self) { self.exit = true }
}

impl Widget for &FlashcardSet {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(self.name.clone().bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(ratatui::symbols::border::DOUBLE);
        let text = Text::from(self.cards.first().unwrap().side_a.clone());
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf)
    }
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
