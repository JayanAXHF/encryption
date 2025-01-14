use adfgvx_cipher::{decrypt_adfgvx, encrypt_adfgvx};
use cli_log::debug;
use color_eyre::eyre::bail;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use morse_code::{decrypt_morse_code, encrypt_morse_code};
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
use std::fs;
use std::io::{Result, Write};
use tracing::field::debug;
use tui_input::backend::crossterm as backend;
use tui_input::backend::crossterm::EventHandler;
use tui_textarea::TextArea;
use tui_textarea::{Input, Key};
use utils::{generate_keyword_string, remove_punctuation, remove_whitespace};
use vigenere_cipher::{decrypt, generate_cipher};
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

use crate::app::{App, CurrentScreen, EncryptionMethods, Modes, SelectedMode};

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
            Press `Ctrl-C` or `q` to stop running or press `Ctrl-S` to continue. {} using {}
",
                mode, method
            );
            let footer = Paragraph::new(Text::styled(
                footer_string,
                Style::default().fg(Color::Blue),
            ))
            .centered();
            let mut keyword_text_area_block = Block::bordered().title("Keyword");
            let mut text_mode = String::new();
            match app.mode.selected_mode {
                SelectedMode::Encrypt => {
                    text_mode.push_str("Encrypted Text");
                }
                SelectedMode::Decrypt => {
                    text_mode.push_str("Decrypted Text");
                }
            }
            if app.read_from_file {
                text_mode = String::from("Enter Filename")
            }
            let mut input_text_area_block = Block::bordered().title(text_mode);
            let mut column_key_text_area_block =
                Block::bordered().title("Column Key[num. 1-6 separted by whitespace]");
            match app.currently_editing {
                crate::app::Inputs::Keyword => {
                    keyword_text_area_block = keyword_text_area_block.border_style(Color::Green);
                }
                crate::app::Inputs::InputText => {
                    input_text_area_block = input_text_area_block.border_style(Color::Green);
                }
                crate::app::Inputs::ColumnKey => {
                    column_key_text_area_block =
                        column_key_text_area_block.border_style(Color::Green);
                }
            }
            match app.encryption {
                EncryptionMethods::VigenereCipher => {
                    let split_layout = Layout::new(
                        Direction::Horizontal,
                        vec![Constraint::Percentage(30), Constraint::Percentage(70)],
                    )
                    .split(chunks[1]);
                    let mut textarea = app.keyword_text_area.clone();
                    textarea.set_block(keyword_text_area_block);

                    let mut input_text_area = app.input_text_area.clone();
                    input_text_area.set_block(input_text_area_block);
                    //textarea.input(app.keyword_input.clone());
                    frame.render_widget(&textarea, split_layout[0]);
                    frame.render_widget(&input_text_area, split_layout[1]);
                }
                EncryptionMethods::ADFGVX => {
                    let split_layout = Layout::new(
                        Direction::Horizontal,
                        vec![Constraint::Percentage(30), Constraint::Percentage(70)],
                    )
                    .split(chunks[1]);
                    let mut textarea = app.keyword_text_area.clone();
                    let mut input_text_area = app.input_text_area.clone();
                    let mut column_key_text_area = app.column_key_text_area.clone();
                    textarea.set_block(keyword_text_area_block);
                    input_text_area.set_block(input_text_area_block);
                    column_key_text_area.set_block(column_key_text_area_block);
                    let double_split = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(split_layout[0]);

                    frame.render_widget(&input_text_area, split_layout[1]);
                    //textarea.input(app.keyword_input.clone());
                    frame.render_widget(&textarea, double_split[0]);
                    frame.render_widget(&column_key_text_area, double_split[1]);
                }
                EncryptionMethods::MorseCode => {
                    let mut textarea = app.input_text_area.clone();
                    let mut text_mode = String::from("Enter the text to ");
                    match app.mode.selected_mode {
                        SelectedMode::Encrypt => {
                            text_mode.push_str("encrypt");
                        }
                        SelectedMode::Decrypt => {
                            text_mode.push_str("decrypt");
                        }
                    }
                    textarea.set_block(input_text_area_block);
                    frame.render_widget(&textarea, chunks[1]);
                }
            }
            frame.render_widget(footer, chunks[2]);
        }
        CurrentScreen::SeeingResult => {
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
                "Created by Jayan Sunil github:JayanAXHF. Showing Results\n
            Press `Ctrl-C` or `q` to stop running or press `Ctrl-S` to continue. {} using {}
",
                mode, method
            );
            let footer = Paragraph::new(Text::styled(
                footer_string,
                Style::default().fg(Color::Blue),
            ))
            .centered();
            let mut keyword_text_area_block = Block::bordered().title("Keyword");
            let mut text_mode = String::new();
            match app.mode.selected_mode {
                SelectedMode::Encrypt => {
                    text_mode.push_str("Encrypted Text");
                }
                SelectedMode::Decrypt => {
                    text_mode.push_str("Decrypted Text");
                }
            }
            if app.read_from_file {
                text_mode = String::from("Enter Filename")
            }
            let mut input_text_area_block = Block::bordered().title(text_mode);
            let mut column_key_text_area_block =
                Block::bordered().title("Column Key[num. 1-6 separated by whitespace]");
            match app.currently_editing {
                crate::app::Inputs::Keyword => {
                    keyword_text_area_block = keyword_text_area_block.border_style(Color::Green);
                }
                crate::app::Inputs::InputText => {
                    input_text_area_block = input_text_area_block.border_style(Color::Green);
                }
                crate::app::Inputs::ColumnKey => {
                    column_key_text_area_block =
                        column_key_text_area_block.border_style(Color::Green);
                }
            }
            app.plaintext = remove_punctuation(&remove_whitespace(&mut app.plaintext.clone()));
            app.encrypted_string =
                remove_punctuation(&remove_whitespace(&mut app.encrypted_string.clone()));
            match app.encryption {
                EncryptionMethods::VigenereCipher => {
                    debug!(
                        "data: {:#?} ---- {:#?} --- {:#?} --- {:#?} --- {:?}",
                        app.keyword.clone(),
                        app.plaintext.clone(),
                        app.encrypted_string.clone(),
                        generate_keyword_string(&mut app.keyword, app.encrypted_string.len()).len(),
                        app.encrypted_string.len()
                    );

                    let output_text = match app.mode.selected_mode {
                        SelectedMode::Encrypt => generate_cipher(
                            app.plaintext.clone(),
                            generate_keyword_string(&mut app.keyword.clone(), app.plaintext.len()),
                        ),
                        SelectedMode::Decrypt => {
                            decrypt(app.encrypted_string.clone(), app.keyword.clone(), false)
                        }
                    };
                    match app.mode.selected_mode {
                        SelectedMode::Encrypt => {
                            app.encrypted_string = output_text.clone();
                        }
                        SelectedMode::Decrypt => {
                            app.plaintext = output_text.clone();
                        }
                    }
                    let split_layout = Layout::new(
                        Direction::Horizontal,
                        vec![Constraint::Percentage(30), Constraint::Percentage(70)],
                    )
                    .split(chunks[1]);
                    let mut textarea = TextArea::new(vec![app.keyword.clone()]);
                    textarea.set_block(keyword_text_area_block);

                    let mut input_text_area = TextArea::new(
                        output_text
                            .chars()
                            .collect::<Vec<char>>()
                            .chunks(100)
                            .map(|chunk| chunk.iter().collect())
                            .collect(),
                    );
                    input_text_area.set_block(input_text_area_block);
                    //textarea.input(app.keyword_input.clone());
                    frame.render_widget(&textarea, split_layout[0]);
                    frame.render_widget(&input_text_area, split_layout[1]);
                }
                EncryptionMethods::ADFGVX => {
                    let split_layout = Layout::new(
                        Direction::Horizontal,
                        vec![Constraint::Percentage(30), Constraint::Percentage(70)],
                    )
                    .split(chunks[1]);
                    let column_key = app.column_key.clone();
                    debug!("{:?} -- Column Key", column_key);

                    let output_text = match app.mode.selected_mode {
                        SelectedMode::Encrypt => encrypt_adfgvx(
                            app.plaintext.clone(),
                            app.keyword.clone(),
                            app.column_key.clone(),
                        ),
                        SelectedMode::Decrypt => decrypt_adfgvx(
                            app.encrypted_string.clone(),
                            app.keyword.clone(),
                            app.column_key.clone(),
                        ),
                    };
                    match app.mode.selected_mode {
                        SelectedMode::Encrypt => {
                            app.encrypted_string = output_text.clone();
                        }
                        SelectedMode::Decrypt => app.plaintext = output_text.clone(),
                    }
                    let mut textarea = TextArea::new(vec![app.keyword.clone()]);
                    textarea.set_block(keyword_text_area_block);
                    let mut input_text_area = TextArea::new(vec![output_text]);

                    let mut column_key_text_area =
                        TextArea::from(app.column_key.iter().map(|f| f.to_string()));

                    input_text_area.set_block(input_text_area_block);
                    column_key_text_area.set_block(column_key_text_area_block);
                    let double_split = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(split_layout[0]);

                    frame.render_widget(&input_text_area, split_layout[1]);
                    //textarea.input(app.keyword_input.clone());
                    frame.render_widget(&textarea, double_split[0]);
                    frame.render_widget(&column_key_text_area, double_split[1]);
                }
                EncryptionMethods::MorseCode => {
                    let mut textarea = app.input_text_area.clone();
                    let mut text_mode = String::from("Enter the text to ");
                    let output_text = match app.mode.selected_mode {
                        SelectedMode::Encrypt => {
                            text_mode.push_str("encrypt");
                            encrypt_morse_code(app.plaintext.clone())
                        }
                        SelectedMode::Decrypt => {
                            text_mode.push_str("decrypt");
                            decrypt_morse_code(app.encrypted_string.clone())
                        }
                    };

                    match app.mode.selected_mode {
                        SelectedMode::Encrypt => {
                            app.encrypted_string = output_text.clone();
                        }
                        SelectedMode::Decrypt => {
                            app.plaintext = output_text.clone();
                        }
                    }
                    textarea.set_block(input_text_area_block);
                    frame.render_widget(&textarea, chunks[1]);
                }
            }
            frame.render_widget(footer, chunks[2]);
        }
        _ => {}
    }
    Ok(())
}
