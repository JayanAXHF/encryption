use serde_json::{json, Value};
use std::{env, io};
use utils::*;
pub fn generate_cipher(plaintext: String, keyword_string: String) -> String {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let numbers: Vec<char> = "0123456789".chars().collect();
    let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    // Combine alphabet and numbers into a single character set
    let alphabet: Vec<char> = alphabet.iter().chain(numbers.iter()).copied().collect();

    let mut encrypted_string = String::new();
    assert_eq!(plaintext.trim().len(), keyword_string.len());
    for (i, char) in plaintext.chars().enumerate() {
        if !alphabet.contains(&char.to_ascii_uppercase()) {
            continue;
        }

        let mut index_plaintext = char
            .to_string()
            .bytes()
            .next()
            .unwrap()
            .to_ascii_uppercase();
        let mut index_key = keyword_string.chars().collect::<Vec<_>>()[i]
            .to_string()
            .bytes()
            .next()
            .unwrap()
            .to_ascii_uppercase();
        if letters.contains(&char.to_ascii_uppercase()) {
            index_plaintext -= 65;
        } else {
            index_plaintext -= 22;
        }
        if letters.contains(&(keyword_string.chars().collect::<Vec<_>>()[i]).to_ascii_uppercase()) {
            index_key -= 65;
        } else {
            index_key -= 22;
        }
        let encrypted_letter = alphabet[((index_plaintext + index_key) % 36) as usize];
        encrypted_string.push(encrypted_letter);
    }

    let mut formatted_string = String::new();
    for (index, char) in encrypted_string.chars().enumerate() {
        if index % 5 == 0 {
            formatted_string.push(' ');
        }
        formatted_string.push(char);
    }
    formatted_string
}

pub fn decrypt(mut encrypted_string: String, keyword: String, ask_for_ai: bool) -> String {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let numbers: Vec<char> = "0123456789".chars().collect();
    let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let alphabet: Vec<char> = alphabet.iter().chain(numbers.iter()).copied().collect();
    let mut decrypted_string = String::new();
    // Combine alphabet and numbers into a single character set
    remove_whitespace(&mut encrypted_string);
    let keyword_string =
        generate_keyword_string(&mut keyword.trim().to_string(), encrypted_string.len());
    for (i, char) in encrypted_string.trim().chars().enumerate() {
        if !alphabet.contains(&char.to_ascii_uppercase()) {
            continue;
        }

        let mut index_encrypted: i32 = char
            .to_string()
            .bytes()
            .next()
            .unwrap()
            .to_ascii_uppercase()
            .into();
        let mut index_key: i32 = keyword_string.chars().collect::<Vec<_>>()[i]
            .to_string()
            .bytes()
            .next()
            .unwrap()
            .to_ascii_uppercase()
            .into();
        if letters.contains(&char.to_ascii_uppercase()) {
            index_encrypted -= 65;
        } else {
            index_encrypted -= 22;
        }
        if letters.contains(&(keyword_string.chars().collect::<Vec<_>>()[i]).to_ascii_uppercase()) {
            index_key -= 65;
        } else {
            index_key -= 22;
        }
        let decrypted_letter = alphabet[(((index_encrypted + 36) - index_key) % 36) as usize];
        decrypted_string.push(decrypted_letter);
    }
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", env::var("GEMINI_API_KEY_RUST").unwrap_or_default());
    let client = reqwest::blocking::Client::new();
    let payload = json!({
        "contents": [
            {
                "parts": [
                    {"text": format!("Format this paragraph: {} and return ONLY THE PARAGRAPH", decrypted_string)}
                ]
            }
        ]
    });

    let mut ai_process = String::new();
    if ask_for_ai {
        println!("Use AI to reformat the string? [requires you to set the environment variable GEMINI_API_KEY_RUST to a valid api key] [y/N]");
        io::stdin()
            .read_line(&mut ai_process)
            .expect("Error reading line");
    }
    if ai_process.trim() == "y" {
        let res = client.post(&url).json(&payload).send();
        let str_res = res
            .expect("Request failed")
            .text()
            .expect("Error reading response content");
        let json_res: Value =
            serde_json::from_str(&str_res[..]).expect("Error converting response to json");
        decrypted_string = json_res["candidates"][0]["content"]["parts"][0]["text"].to_string();
    }
    decrypted_string
}
