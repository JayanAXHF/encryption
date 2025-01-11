use crossterm::cursor::{Hide, Show};
use crossterm::event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::{CrosstermBackend, Stylize};
use ratatui::Terminal;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style,
    },
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::io::{Result, Write};
use tui_input::backend::crossterm as backend;
use tui_input::backend::crossterm::EventHandler;
use tui_textarea::TextArea;
use tui_textarea::{Input, Key};
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

use crate::app::{App, CurrentScreen, EncryptionMethods, SelectedMode};

pub fn ui(frame: &mut Frame, app: &mut App) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "
▄█▄    ▄█ █ ▄▄   ▄  █ ▄███▄   █▄▄▄▄ ▄███▄   ██▄
█▀ ▀▄  ██ █   █ █   █ █▀   ▀  █  ▄▀ █▀   ▀  █  █
█   ▀  ██ █▀▀▀  ██▀▀█ ██▄▄    █▀▀▌  ██▄▄    █   █
█▄  ▄▀ ▐█ █     █   █ █▄   ▄▀ █  █  █▄   ▄▀ █  █
▀███▀   ▐  █       █  ▀███▀     █   ▀███▀   ███▀
            ▀     ▀            ▀

",
        Style::default().fg(Color::LightBlue),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);
    match app.current_screen {
        CurrentScreen::ChoosingEncryption => {
            let mut state = app.encryption_methods_list.state.clone();
            let items: Vec<ListItem> = app
                .encryption_methods_list
                .items
                .iter()
                .map(|method| ListItem::from(method.name.clone()))
                .collect();
            let list = List::new(items)
                .block(Block::bordered().title("Choose the Method of Encryption"))
                .highlight_style(SELECTED_STYLE)
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true);
            frame.render_stateful_widget(list, chunks[1], &mut state);
            let footer = Paragraph::new(Text::styled(
                "Created by Jayan Sunil github:JayanAXHF\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.",
                Style::default().fg(Color::Blue),
            ))
            .centered();

            frame.render_widget(footer, chunks[2]);
        }
        CurrentScreen::ChoosingMode => {
            let mut state = app.mode.state.clone();
            let items: Vec<ListItem> = app
                .mode
                .items
                .iter()
                .map(|method| ListItem::from(method.name.clone()))
                .collect();
            let list = List::new(items)
                .block(Block::bordered().title("Choose Mode"))
                .highlight_style(SELECTED_STYLE)
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, chunks[1], &mut state);
            let mode = match app.encryption {
                EncryptionMethods::VigenereCipher => "Using Vigenere Cipher",
                EncryptionMethods::ADFGVX => "using ADFGVX Cipher",
                EncryptionMethods::MorseCode => "Using Morse code",
            };
            let footer_string = format!(
                "Created by Jayan Sunil github:JayanAXHF\n
            Press `Esc`, `Ctrl-C` or `q` to stop running. {}
",
                mode
            );
            let footer = Paragraph::new(Text::styled(
                footer_string,
                Style::default().fg(Color::Blue),
            ))
            .centered();

            frame.render_widget(footer, chunks[2]);
        }
        CurrentScreen::InputtingValues => {
            let method = match app.encryption {
                EncryptionMethods::VigenereCipher => "Vigenere Cipher",
                EncryptionMethods::ADFGVX => "ADFGVX Cipher",
                EncryptionMethods::MorseCode => "Morse code",
            };
            let mode = match app.mode.selected_mode {
                SelectedMode::Encrypt => "Encrypting",
                SelectedMode::Decrypt => "Decrypting",
            };
            let footer_string = format!(
                "Created by Jayan Sunil github:JayanAXHF\n
            Press `Esc`, `Ctrl-C` or `q` to stop running. {} using {}
",
                mode, method
            );
            let footer = Paragraph::new(Text::styled(
                footer_string,
                Style::default().fg(Color::Blue),
            ))
            .centered();
            let mut textarea = app.keyword_text_area.clone();
            textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Crossterm Minimal Example"),
            );

            textarea.input(app.keyword_input.clone());
            frame.render_widget(&textarea, chunks[1]);
            frame.render_widget(footer, chunks[2]);
        }
        _ => {}
    }
    Ok(())
}
