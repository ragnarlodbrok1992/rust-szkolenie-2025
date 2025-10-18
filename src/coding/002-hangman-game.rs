// Implement basic hangman game with CLI interface
// 1. Program draws random word from some dictionary.
// 2. Prompts user about how much letters are in the guessed word.
// 3. User can input one character.
// 4. If it's a hit - it shows guessed characters.
// 5. If not - adds a piece to a hangman.
// 6. Win condition - guess the word. Lose condition - no hits for all tries.
//
// Hangmang ASCII art to help:
//   +---+
//   |   |
//   O   |
//  /|\  |
//  / \  |
//       |

use std::io;
use std::io::{Error, Write};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

fn main() {
    let provider = HardcodedDictionaryProvider::new(vec![
        "rust".to_string(),
        "hangman".to_string(),
        "programming".to_string(),
        "development".to_string(),
        "challenge".to_string(),
    ]);

    let game = Game::new(provider);
    match game {
        Ok(game) => {
            if let Err(err) = game.run() {
                eprintln!("Game error: {:?}", err);
            }
        },
        Err(err) => match err {
            GameError::EmptyDictionary => eprintln!("The dictionary is empty."),
            GameError::IoError(err) => eprintln!("I/O error: {}", err),
        }
    }
}

pub struct Setup {
    word_to_guess: String,
}

impl Setup {
    pub fn new(words: Vec<String>) -> Result<Self, GameError> {
        if words.is_empty() {
            return Err(GameError::EmptyDictionary);
        }

        let word_to_guess = Self::pick_word(words);

        Ok(Setup { word_to_guess })
    }

    pub fn start(self) -> Game {
        Game::Running(
            RunningGame {
                word_to_guess: self.word_to_guess,
                guessed_letters: Vec::new(),
                wrong_letters: Vec::new(),
                remaining_attempts: 6,
            }
        )
    }

    fn pick_word(dictionary: Vec<String>) -> String {
        let mut rng = StdRng::from_rng(&mut rand::rng());
        let word_index = rng.random_range(0..dictionary.len());

        dictionary[word_index].clone()
    }
}

pub struct RunningGame {
    word_to_guess: String,
    guessed_letters: Vec<char>,
    wrong_letters: Vec<char>,
    remaining_attempts: u8,
}

impl RunningGame {
    pub fn guess_letter(mut self, letter: char) -> Game {
        let letter = letter.to_ascii_lowercase();
        if self.word_to_guess.to_lowercase().contains(letter) {
            self.guessed_letters.push(letter);
        } else {
            self.wrong_letters.push(letter);
            self.remaining_attempts -= 1;
        }

        if self.is_won() {
            Game::Finished(GameOverReason::Won {word: self.word_to_guess})
        } else if self.is_lost() {
            Game::Finished(GameOverReason::Lost {word: self.word_to_guess})
        } else {
            Game::Running(self)
        }
    }

    pub fn guessed_letters(&self) -> &Vec<char> {
        &self.guessed_letters
    }

    pub fn remaining_attempts(&self) -> u8 {
        self.remaining_attempts
    }

    pub fn wrong_letters(&self) -> &Vec<char> {
        &self.wrong_letters
    }

    fn is_won(&self) -> bool {
        self.word_to_guess
            .to_lowercase()
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }

    fn is_lost(&self) -> bool {
        self.remaining_attempts == 0
    }
}

pub enum GameOverReason {
    Won { word: String },
    Lost { word: String },
}

pub enum Game {
    Initializing(Setup),
    Running(RunningGame),
    Finished(GameOverReason),
}

impl Game {
    pub fn new(dictionary_provider: impl DictionaryProvider) -> Result<Self, GameError> {
        let words = dictionary_provider.get_words();
        let setup = Setup::new(words)?;

        Ok(Game::Initializing(setup))
    }

    pub fn run(self) -> Result<(), GameError> {
        println!("Welcome to Hangman!");
        let mut current_game = self;

        loop {
            current_game = match current_game {
                Game::Initializing(setup) => {
                    GameRenderer::render_setup(&setup)?;
                    setup.start()
                }
                Game::Running(running_game) => {
                    GameRenderer::render(&running_game);
                    let letter = GameInputHandler::get_letter_from_user()?;
                    running_game.guess_letter(letter)
                }
                Game::Finished(r) => {
                    match r {
                        GameOverReason::Won {word} => println!("Congratulations! You've won! the word was '{}'.", word),
                        GameOverReason::Lost {word} => println!("Game over! You've lost! the word was '{}'.", word),
                    }

                    break Ok(());
                }
            };
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    EmptyDictionary,
    IoError(Error),
}

impl From<Error> for GameError {
    fn from(error: Error) -> Self {
        GameError::IoError(error)
    }
}

pub struct GameInputHandler;

impl GameInputHandler {
    pub fn get_confirmation_from_user(question: String) -> Result<bool, GameError> {
        print!("{} (y/n): ", question);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim().to_lowercase();
        match trimmed.as_str() {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            _ => {
                println!("Please enter 'y' or 'n'.");
                Self::get_confirmation_from_user(question)
            },
        }
    }

    pub fn get_letter_from_user() -> Result<char, GameError> {
        print!("Please enter a letter to guess: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();
        if trimmed.len() != 1 {
            println!("Please enter a single letter.");
            return Self::get_letter_from_user();
        }

        if let Some(letter) = trimmed.chars().next() {
            if !letter.is_alphabetic() {
                println!("Please enter a valid alphabetic character.");
                return Self::get_letter_from_user();
            }

            Ok(letter)
        } else {
            println!("Please enter a valid letter.");
            Self::get_letter_from_user()
        }
    }
}

pub struct GameRenderer;

impl GameRenderer {
    pub fn render_setup(state: &Setup) -> Result<(), GameError> {
        println!("The word to guess has {} letters.", state.word_to_guess.len());
        GameInputHandler::get_confirmation_from_user("Are you ready to start?".to_string())?;

        Ok(())
    }

    pub fn render(state: &RunningGame) {
        print!("\x1B[2J\x1B[1;1H");

        Self::render_hangman(state.wrong_letters.len());
        println!("\n    {}\n", Self::render_word(state));
        println!("The word to guess has {} letters.", state.word_to_guess.len());
        println!("Remaining attempts: {}", state.remaining_attempts());
        println!("Wrong letters: {:?}", state.wrong_letters());
    }

    fn render_hangman(mistakes_made: usize) {
        if let Some(stage) = HANGMAN_STAGES.get(mistakes_made as usize) {
            println!("{}", stage);
        }
    }

    fn render_word(state: &RunningGame) -> String {
        state
            .word_to_guess
            .chars()
            .map(|c| {
                if !c.is_alphabetic() {
                    c
                } else if state.guessed_letters().contains(&c.to_ascii_lowercase()) {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
    }
}

pub trait DictionaryProvider {
    fn get_words(&self) -> Vec<String>;
}

pub struct HardcodedDictionaryProvider {
    words: Vec<String>,
}

impl HardcodedDictionaryProvider {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }
}

impl DictionaryProvider for HardcodedDictionaryProvider {
    fn get_words(&self) -> Vec<String> {
        self.words.clone()
    }
}

const HANGMAN_STAGES: [&str; 7] = [
    r#"
  +---+
  |   |
      |
      |
      |
      |"#,
    r#"
  +---+
  |   |
  O   |
      |
      |
      |"#,
    r#"
  +---+
  |   |
  O   |
  |   |
      |
      |"#,
    r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |"#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |"#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |"#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |"#,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_word() {
        let words = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ];
        let setup = Setup::new(words.clone()).unwrap();
        assert!(words.contains(&setup.word_to_guess));
    }

    #[test]
    fn test_guess_letter_hit() {
        let running_game = RunningGame {
            word_to_guess: "rust".to_string(),
            guessed_letters: vec![],
            wrong_letters: vec![],
            remaining_attempts: 6,
        };

        let next_game = running_game.guess_letter('r');
        if let Game::Running(rg) = next_game {
            assert_eq!(rg.guessed_letters, vec!['r']);
            assert_eq!(rg.wrong_letters.len(), 0);
            assert_eq!(rg.remaining_attempts, 6);
        } else {
            panic!("Expected Running game state");
        }
    }

    #[test]
    fn test_guess_letter_miss() {
        let running_game = RunningGame {
            word_to_guess: "rust".to_string(),
            guessed_letters: vec![],
            wrong_letters: vec![],
            remaining_attempts: 6,
        };

        let next_game = running_game.guess_letter('x');
        if let Game::Running(rg) = next_game {
            assert_eq!(rg.guessed_letters.len(), 0);
            assert_eq!(rg.wrong_letters, vec!['x']);
            assert_eq!(rg.remaining_attempts, 5);
        } else {
            panic!("Expected Running game state");
        }
    }

    #[test]
    fn test_is_won() {
        let running_game = RunningGame {
            word_to_guess: "rust".to_string(),
            guessed_letters: vec!['r', 'u', 's', 't'],
            wrong_letters: vec![],
            remaining_attempts: 6,
        };

        assert!(running_game.is_won());
    }

    #[test]
    fn test_is_lost() {
        let running_game = RunningGame {
            word_to_guess: "rust".to_string(),
            guessed_letters: vec![],
            wrong_letters: vec!['a', 'b', 'c', 'd', 'e', 'f'],
            remaining_attempts: 0,
        };

        assert!(running_game.is_lost());
    }

    #[test]
    fn test_empty_dictionary() {
        let result = Setup::new(vec![]);
        assert!(matches!(result, Err(GameError::EmptyDictionary)));
    }

    #[test]
    fn test_hardcoded_dictionary_provider() {
        let words = vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
        ];
        let provider = HardcodedDictionaryProvider::new(words.clone());
        let retrieved_words = provider.get_words();
        assert_eq!(retrieved_words, words);
    }

    #[test]
    fn test_render_word() {
        let running_game = RunningGame {
            word_to_guess: "hangman".to_string(),
            guessed_letters: vec!['a', 'n'],
            wrong_letters: vec![],
            remaining_attempts: 6,
        };
        let rendered_word = GameRenderer::render_word(&running_game);
        assert_eq!(rendered_word, "_an__an");
    }
}
