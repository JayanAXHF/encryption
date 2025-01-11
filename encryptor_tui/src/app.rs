use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::Backend,
    style::Stylize,
    text::Line,
    widgets::{Block, ListState, Paragraph},
    DefaultTerminal, Frame, Terminal,
};
use std::env;
use tui_textarea::{CursorMove, Input, Key, TextArea};

use crate::ui::ui;

#[derive(Debug)]
pub enum CurrentScreen {
    ChoosingEncryption,
    ChoosingMode,
    InputtingValues,
    SeeingResult,
    Exiting,
}

#[derive(Debug)]
pub enum EncryptionMethods {
    VigenereCipher,
    ADFGVX,
    MorseCode,
}

#[derive(Debug)]
pub struct EncryptionMethod {
    pub name: String,
}
#[derive(Debug)]
pub struct ChosenMethodList {
    pub items: Vec<EncryptionMethod>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct Mode {
    pub name: String,
}

#[derive(Debug)]
pub enum SelectedMode {
    Encrypt,
    Decrypt,
}

#[derive(Debug)]
pub struct Modes {
    pub items: Vec<Mode>,
    pub state: ListState,
    pub selected_mode: SelectedMode,
}

#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub keyword: String,
    pub column_key: Vec<u8>,
    pub plaintext: String,
    pub encrypted_string: String,
    pub write_to_file: bool,
    pub read_from_file: bool,
    pub current_screen: CurrentScreen,
    pub encryption: EncryptionMethods,
    pub encryption_methods_list: ChosenMethodList,
    pub mode: Modes,
    pub keyword_input: Input,
    pub keyword_text_area: TextArea<'a>
}

impl Default for App<'_> {
    fn default() -> Self {
        let read_from_file = env::var("READ_FROM_FILE").is_ok();
        let column_key: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        App {
            running: true,
            keyword: String::new(),
            column_key,
            plaintext: String::new(),
            encrypted_string: String::new(),
            write_to_file: false,
            read_from_file,
            current_screen: CurrentScreen::ChoosingEncryption,
            encryption: EncryptionMethods::VigenereCipher,
            encryption_methods_list: ChosenMethodList {
                items: vec![
                    EncryptionMethod {
                        name: "Vigenere Cipher".to_string(),
                    },
                    EncryptionMethod {
                        name: "ADFGVX Cipher".to_string(),
                    },
                    EncryptionMethod {
                        name: "Morse Code".to_string(),
                    },
                ],
                state: ListState::default(),
            },
            mode: Modes {
                items: vec![
                    Mode {
                        name: "Encryption Mode".to_string(),
                    },
                    Mode {
                        name: "Decryption Mode".to_string(),
                    },
                ],
                state: ListState::default(),
                selected_mode: SelectedMode::Encrypt,
            },
            keyword_input: Input {
                key: Key::Char('a'),
                ctrl: true,
                alt: false,
                shift: false,
            },
            keyword_text_area: TextArea::default()
        }
    }
}

impl App<'_> {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    fn select_none(&mut self) {
        match self.current_screen {
            CurrentScreen::ChoosingEncryption => {
                self.encryption_methods_list.state.select(None);
            }
            CurrentScreen::ChoosingMode => {}
            _ => {}
        }
    }

    fn select_next(&mut self) {
        match self.current_screen {
            CurrentScreen::ChoosingEncryption => {
                self.encryption_methods_list.state.select_next();
            }
            CurrentScreen::ChoosingMode => {
                self.mode.state.select_next();
            }
            _ => {}
        }
    }
    fn select_previous(&mut self) {
        match self.current_screen {
            CurrentScreen::ChoosingEncryption => {
                self.encryption_methods_list.state.select_previous();
            }
            CurrentScreen::ChoosingMode => {
                self.mode.state.select_previous();
            }
            _ => {}
        }
    }

    fn select_first(&mut self) {
        match self.current_screen {
            CurrentScreen::ChoosingEncryption => {
                self.encryption_methods_list.state.select_first();
            }
            CurrentScreen::ChoosingMode => {}
            _ => {}
        }
    }

    fn select_last(&mut self) {
        match self.current_screen {
            CurrentScreen::ChoosingEncryption => {
                self.encryption_methods_list.state.select_last();
            }
            CurrentScreen::ChoosingMode => {}
            _ => {}
        }
    }
    fn select_current(&mut self) {
        match self.current_screen {
            CurrentScreen::ChoosingEncryption => {
                if self.encryption_methods_list.state.selected().is_some() {
                    self.encryption = match self.encryption_methods_list.state.selected() {
                        Some(0) => EncryptionMethods::VigenereCipher,
                        Some(1) => EncryptionMethods::ADFGVX,
                        Some(2) => EncryptionMethods::MorseCode,
                        _ => EncryptionMethods::VigenereCipher,
                    };
                    self.current_screen = CurrentScreen::ChoosingMode
                }
            }
            CurrentScreen::ChoosingMode => {
                if self.mode.state.selected().is_some() {
                    self.mode.selected_mode = match self.mode.state.selected() {
                        Some(0) => SelectedMode::Encrypt,
                        Some(1) => SelectedMode::Decrypt,
                        _ => SelectedMode::Encrypt,
                    };
                    self.current_screen = CurrentScreen::InputtingValues;
                }
            }
            _ => {}
        }
    }
    /// Run the application's main loop.
    pub fn run<B: Backend>(mut self, mut terminal: &mut Terminal<B>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| ui(frame, &mut self).expect("REASON"))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc` to stop running.";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match self.current_screen {
                        CurrentScreen::InputtingValues => {
                            match key.code {
                                KeyCode::Char(value) => {
                                    self.keyword_text_area.insert_char(value);
                                    self.keyword_text_area.move_cursor(CursorMove::Forward);
                                },
                                KeyCode::Backspace => {
                                    self.keyword_text_area.delete_char();
                                    self.keyword_text_area.move_cursor(CursorMove::Back);
                                },
                                KeyCode::Esc => {self.quit();}
                                _ => {}
                            }
                        }
                        _ => self.on_key_event(key),
                    }
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Esc {
            self.quit();
        }

        if let CurrentScreen::ChoosingEncryption | CurrentScreen::ChoosingMode = self.current_screen
        {
            match key.code {
                KeyCode::Char('h') | KeyCode::Left => self.select_none(),
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => self.select_first(),
                KeyCode::Char('G') | KeyCode::End => self.select_last(),
                KeyCode::Enter => {
                    self.select_current();
                }
                _ => {}
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
