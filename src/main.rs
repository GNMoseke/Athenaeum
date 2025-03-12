use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use flashcards::*;
use rand::{rng, seq::SliceRandom};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};
use std::{
    fs::{self},
    path::PathBuf,
    time::Duration,
};

pub mod flashcards;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Simple TUI flashcards.")]
struct Args {
    #[arg(short('f'), long, help = "Directory to find flashcard sets.")]
    sets_dir: String,
    #[arg(
        short,
        long,
        help = "Name of the set to run. Case insensitive, no file extension."
    )]
    set: String,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Show flashcard contents in all caps."
    )]
    capitalize: bool,
    #[arg(
        short('r'),
        long,
        default_value_t = false,
        help = "Shuffle set before starting."
    )]
    shuffle: bool,
}

fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    let args = Args::parse();
    let mut model = App::new(args.sets_dir, args.set, args.capitalize, args.shuffle);

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
    //all_sets: HashSet<FlashcardSet>,
    current_set: FlashcardSet,
    current_card: Flashcard,
}

#[derive(PartialEq)]
enum Message {
    //ChooseSet,
    //Confirm,
    NextCard,
    PreviousCard,
    //SetStats,
    Flip,
    Quit,
}

fn update(model: &mut App, msg: Message) -> Option<Message> {
    match msg {
        //Message::ChooseSet => todo!(),
        //Message::Confirm => todo!(),
        Message::NextCard => {
            if let Some(next_card) = model.current_set.next_card() {
                model.current_card = next_card.clone();
                return None;
            }
            //return Some(Message::SetStats);
        }
        Message::PreviousCard => {
            if let Some(next_card) = model.current_set.prev_card() {
                model.current_card = next_card.clone();
            }
        }
        Message::Flip => {
            model.current_card.flip();
        }
        //Message::SetStats => todo!(),
        Message::Quit => model.exit = true,
    };
    None
}

fn view(model: &mut App, frame: &mut Frame) {
    let color = match model.current_card.current_side {
        CurrentSide::Front => Color::LightBlue,
        CurrentSide::Back => Color::LightMagenta,
    };

    let title = Line::from(
        model
            .current_set
            .name
            .to_uppercase()
            .clone()
            .bold()
            .underlined()
            .italic(),
    );
    let block = Block::bordered()
        .title(title.centered())
        .padding(Padding::new(4, 4, 4, 4))
        .border_set(ratatui::symbols::border::DOUBLE)
        .fg(color);
    let text = Text::from(model.current_card.current_side());
    let area = center(frame.area(), Constraint::Length(45), Constraint::Length(12));
    let flashcard = Paragraph::new(text).centered().block(block);
    frame.render_widget(flashcard, area);
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
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
    pub fn new(sets_dir_path: String, set: String, capitalize: bool, shuffle: bool) -> Self {
        let all_sets = find_all_sets(sets_dir_path);
        // TODO: lots of error handling here instead of panicking
        let sets = all_sets.unwrap();
        let matching_sets = sets
            .iter()
            .filter(|s| s.0.to_lowercase() == set.to_lowercase())
            .collect::<Vec<&(String, PathBuf)>>();
        let set_path = matching_sets.first().unwrap();
        let mut current_set = parse_set(
            &fs::read_to_string(set_path.1.clone()).unwrap(),
            set_path.0.clone(),
            capitalize,
        );
        if shuffle {
            current_set.cards.shuffle(&mut rng())
        }
        App {
            exit: false,
            //all_sets: HashSet::new(),
            current_set: current_set.clone(),
            current_card: current_set.cards.first().unwrap().clone(),
        }
    }
}
