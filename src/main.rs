use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{collections::HashSet, ffi::OsStr, fs::read_dir, io, path::PathBuf, time::Duration};

fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    let mut model = App::new();
    while !model.exit {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event()?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    ratatui::restore();
    Ok(())
}

struct App {
    exit: bool,
    all_sets: HashSet<FlashcardSet>,
    current_set: FlashcardSet,
    current_card: Flashcard,
}

#[derive(PartialEq)]
enum Message {
    ChooseSet,
    Confirm,
    NextCard,
    PreviousCard,
    SetStats,
    Flip,
    Quit,
}

fn update(model: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::ChooseSet => todo!(),
        Message::Confirm => todo!(),
        Message::NextCard => {
            if let Some(next_card) = model.current_set.next_card() {
                model.current_card = next_card.clone();
                return None;
            }
            return Some(Message::SetStats);
        }
        Message::PreviousCard => {
            if let Some(next_card) = model.current_set.prev_card() {
                model.current_card = next_card.clone();
            }
        }
        Message::Flip => {
            model.current_card.flip();
        }
        Message::SetStats => todo!(),
        Message::Quit => model.exit = true,
    };
    None
}

fn view(model: &mut App, frame: &mut Frame) {
    let title = Line::from(model.current_set.name.clone().bold());
    let block = Block::bordered()
        .title(title.centered())
        .border_set(ratatui::symbols::border::DOUBLE);
    let text = Text::from(model.current_card.current_side());
    let flashcard = Paragraph::new(text).centered().block(block);
    frame.render_widget(flashcard, frame.area());
}

fn handle_event() -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('n') => Some(Message::NextCard),
        KeyCode::Char('p') => Some(Message::PreviousCard),
        KeyCode::Char(' ') => Some(Message::Flip),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

impl App {
    pub fn new() -> Self {
        let set = make_test_set();
        App {
            exit: false,
            all_sets: HashSet::new(),
            current_set: set.clone(),
            current_card: set.cards.first().unwrap().clone(),
        }
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
                    front: a.trim().to_string(),
                    back: b.trim().to_string(),
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

#[derive(Debug)]
struct OneRunStats {
    correct: HashSet<Flashcard>,
    incorrect: HashSet<Flashcard>,
}

#[derive(Debug, PartialEq, Clone)]
struct FlashcardSet {
    name: String,
    cards: Vec<Flashcard>,
    current_card_idx: usize,
}

impl FlashcardSet {
    fn current_card(&mut self) -> Option<&mut Flashcard> {
        self.cards.get_mut(self.current_card_idx)
    }
    fn next_card(&mut self) -> Option<&Flashcard> {
        self.current_card_idx += 1;
        self.cards.get(self.current_card_idx)
    }

    fn prev_card(&mut self) -> Option<&Flashcard> {
        self.current_card_idx -= 1;
        self.cards.get(self.current_card_idx)
    }
}
#[derive(Debug, PartialEq, Clone)]
struct Flashcard {
    front: String,
    back: String,
    current_side: CurrentSide,
}

#[derive(Debug, PartialEq, Clone)]
enum CurrentSide {
    Front,
    Back,
}

impl Flashcard {
    fn current_side(&self) -> String {
        match self.current_side {
            CurrentSide::Back => self.back.clone(),
            CurrentSide::Front => self.front.clone(),
        }
    }

    fn flip(&mut self) {
        match self.current_side {
            CurrentSide::Back => self.current_side = CurrentSide::Front,
            CurrentSide::Front => self.current_side = CurrentSide::Back,
        }
    }
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
                    front: "foo".to_string(),
                    back: "bar".to_string(),
                    current_side: CurrentSide::Front
                },
                Flashcard {
                    front: "baz".to_string(),
                    back: "thenextone".to_string(),
                    current_side: CurrentSide::Front
                }
            ],
            current_card_idx: 0
        }
    );
}

fn make_test_set() -> FlashcardSet {
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
}
