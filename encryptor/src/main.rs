#![allow(unused_assignments)]
use adfgvx_cipher::*;
use dialoguer::Input;
use inquire::error::InquireError;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self};
use utils::*;
use vigenere_cipher::*;
fn main() {
    println!(
        "{:^100}",
        "
▄█▄    ▄█ █ ▄▄   ▄  █ ▄███▄   █▄▄▄▄ ▄███▄   ██▄
█▀ ▀▄  ██ █   █ █   █ █▀   ▀  █  ▄▀ █▀   ▀  █  █
█   ▀  ██ █▀▀▀  ██▀▀█ ██▄▄    █▀▀▌  ██▄▄    █   █
█▄  ▄▀ ▐█ █     █   █ █▄   ▄▀ █  █  █▄   ▄▀ █  █
▀███▀   ▐  █       █  ▀███▀     █   ▀███▀   ███▀
            ▀     ▀            ▀

"
    );
    println!("-----------------------------------------------------------------------------------------------------------------");
    println!("-----------------------------------------------------------------------------------------------------------------");
    let available_methods = HashMap::from([(0, vigenere as fn()), (1, adfgvx as fn())]);
    let items = vec!["Vigenère Cipher", "ADFGVX Cipher"];

    let ans: Result<&str, InquireError> =
        inquire::Select::new("What ecryption method do you choose?", items.clone()).prompt();
    let mut selection: usize = 0;
    match ans {
        Ok(choice) => selection = items.iter().position(|x| &choice == x).unwrap(),
        Err(_) => println!("There was an error, please try again"),
    }
    available_methods.get(&selection).unwrap()()
}
fn print_table() {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let numbers: Vec<char> = "0123456789".chars().collect();

    // Combine alphabet and numbers into a single character set
    let full_set: Vec<char> = alphabet.iter().chain(numbers.iter()).copied().collect();

    // Number of characters in the combined set
    let set_length = full_set.len();

    // Generate the Vigenère table
    let mut vigenere_table: Vec<Vec<char>> = Vec::new();

    for i in 0..set_length {
        let mut row: Vec<char> = Vec::new();
        for j in 0..set_length {
            row.push(full_set[(i + j) % set_length]);
        }
        vigenere_table.push(row);
    }

    // Print the column headers
    print!("   | "); // Padding for the row header and column separator
    for &c in &full_set {
        print!("{:>2} ", c);
    }
    println!();

    // Print a separator line
    print!("---+-"); // Row header and column separator
    for _ in 0..set_length {
        print!("---");
    }
    println!();

    // Print the table rows with row headers and a vertical separator
    for (i, row) in vigenere_table.iter().enumerate() {
        print!("{:>2} | ", full_set[i]); // Row header with a vertical separator
        for &c in row {
            print!("{:>2} ", c);
        }
        println!();
    }
}

fn vigenere() {
    print_table();
    println!("-----------------------------------------------------------------------------------------------------------------");
    println!("-----------------------------------------------------------------------------------------------------------------\n\n");
    let mut mode = String::new();
    println!("Encryption Mode/Decryption Mode [e/D]:");
    io::stdin()
        .read_line(&mut mode)
        .expect("Error reading input");

    println!("Enter Keyword:");
    let mut keyword = String::new();
    io::stdin()
        .read_line(&mut keyword)
        .expect("Error reading line");
    println!("{:?}", mode.trim());
    if mode.trim() == "e" {
        let mut plaintext = String::new();
        if env::var("READ_FROM_FILE").is_ok() {
            println!("\nEnter filename/path:");
            let mut filepath = String::new();
            io::stdin()
                .read_line(&mut filepath)
                .expect("Error reading input");
            plaintext = fs::read_to_string(filepath.trim()).expect("error reading file");
        } else {
            println!("Enter plaintext:");
            io::stdin()
                .read_line(&mut plaintext)
                .expect("Error reading input from user.");
        }
        remove_whitespace(&mut plaintext);
        remove_whitespace(&mut keyword);
        let plaintext = remove_punctuation(&plaintext);
        let mut keyword = remove_punctuation(&keyword);
        println!("{}", plaintext);

        let keyword_string = generate_keyword_string(&mut keyword, plaintext.len());

        let encrypted_string = generate_cipher(plaintext, keyword_string);
        println!("Encrypted text:");
        println!("-----------------------------------------------------------------------------------------------------------------");
        println!("{}", encrypted_string.trim());
        println!("-----------------------------------------------------------------------------------------------------------------\n\n");

        let items = vec!["Yes", "No"];

        let ans = inquire::Select::new("Write the encrypted text to file?", items.clone())
            .prompt()
            .unwrap();

        if ans == "Yes" {
            let mut filename: String = String::new();
            println!("Name the output file [press ENTER for default]");
            io::stdin()
                .read_line(&mut filename)
                .expect("Error reading input");
            filename = filename.trim().to_string();

            if filename.is_empty() {
                filename = "encrypted_text.txt".to_string();
            } else {
                filename.push_str(".txt");
            }

            let temp = fs::write(filename, encrypted_string.trim());
            match temp {
                Ok(_) => {
                    println!("File created successfully")
                }
                _ => {
                    println!("Error while writing the output to file:")
                }
            }
        }
    } else {
        println!("-----------------------------------------------------------------------------------------------------------------");
        println!("Ensure that the encrypted_string contains only [A-Z] and [0-9]");
        println!("-----------------------------------------------------------------------------------------------------------------\n\n");
        let mut encrypted_string = String::new();
        if env::var("READ_FROM_FILE").is_ok() {
            println!("\nEnter filename/path:");
            let mut filepath = String::new();
            io::stdin()
                .read_line(&mut filepath)
                .expect("Error reading input");
            encrypted_string = fs::read_to_string(filepath.trim()).expect("error reading file");
        } else {
            println!("Enter encrypted text:");
            io::stdin()
                .read_line(&mut encrypted_string)
                .expect("Error reading input from user.");
        }
        let decrypted_string = decrypt(encrypted_string, keyword);
        println!("Decrypted text:");
        println!("-----------------------------------------------------------------------------------------------------------------");
        println!("{}", decrypted_string.trim());

        println!("-----------------------------------------------------------------------------------------------------------------\n\n");
        let items = vec!["Yes", "No"];

        let ans = inquire::Select::new("Write the decrypted text to file?", items.clone())
            .prompt()
            .unwrap();

        if ans == "Yes" {
            let mut filename: String = String::new();
            println!("Name the output file [press ENTER for default]");
            io::stdin()
                .read_line(&mut filename)
                .expect("Error reading input");
            filename = filename.trim().to_string();

            if filename.is_empty() {
                filename = "decrypted_string.txt".to_string();
            } else {
                filename.push_str(".txt");
            }

            let temp = fs::write(filename, decrypted_string.trim());
            match temp {
                Ok(_) => {
                    println!("File created successfully")
                }
                _ => {
                    println!("Error while writing the output to file:")
                }
            }
        }
    }
}

fn adfgvx() {
    let items = vec!["Encryption Mode", "Decryption Mode"];

    let mode = inquire::Select::new("Select Mode", items.clone())
        .prompt()
        .unwrap();

    let mut plaintext = String::new();
    if env::var("READ_FROM_FILE").is_ok() {
        println!("\nEnter filename/path: ");
        let mut filepath = String::new();
        io::stdin()
            .read_line(&mut filepath)
            .expect("Error reading input");
        plaintext = fs::read_to_string(filepath.trim())
            .expect("error reading file")
            .trim()
            .to_string();
    } else {
        println!("\nEnter plaintext: ");
        io::stdin()
            .read_line(&mut plaintext)
            .expect("Error reading input from user.");
    }

    let key: String = Input::new()
        .with_prompt("Enter encryption key")
        .interact_text()
        .unwrap();
    let column_key: String = Input::new()
        .with_prompt("Enter second key (Please enter numbers from 1-6 separated by whitespaces)")
        .interact_text()
        .unwrap();

    let plaintext = remove_punctuation(&plaintext);
    let mut key = remove_punctuation(&key);
    let key = remove_whitespace(&mut key);

    let column_key: Vec<u8> = column_key
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let square = polybius_square(key.clone());
    let square = polybius_to_string(square);
    if mode == "Encryption Mode" {
        let encrypted_string = encrypt_adfgvx(plaintext, key.clone(), column_key);

        println!("\n{}", square);
        println!("-----------------------------------------------------------------------------------------------------------------");
        println!("{}", encrypted_string);
        println!("-----------------------------------------------------------------------------------------------------------------");
        let items = vec!["Yes", "No"];

        let ans = inquire::Select::new("Write the encrypted text to file?", items.clone())
            .prompt()
            .unwrap();

        if ans == "Yes" {
            let mut filename: String = String::new();
            println!("Name the output file [press ENTER for default]");
            io::stdin()
                .read_line(&mut filename)
                .expect("Error reading input");
            filename = filename.trim().to_string();

            if filename.is_empty() {
                filename = "encrypted_text.txt".to_string();
            } else {
                filename.push_str(".txt");
            }
            /*match custom_filename {
                Ok(file) => filename = file,
                _ => filename = "encrypted_text.txt",
            }*/
            let temp = fs::write(filename, encrypted_string.trim());
            match temp {
                Ok(_) => {
                    println!("File created successfully")
                }
                _ => {
                    println!("Error while writing the output to file:")
                }
            }
        }
    } else {
        let decrypted_string = decrypt_adfgvx(plaintext, key, column_key);

        println!("\n{}", square);
        println!("-----------------------------------------------------------------------------------------------------------------");
        println!("{}", decrypted_string);
        println!("-----------------------------------------------------------------------------------------------------------------");
        let items = vec!["Yes", "No"];

        let ans = inquire::Select::new("Write the decrypted text to file?", items.clone())
            .prompt()
            .unwrap();

        if ans == "Yes" {
            let mut filename: String = String::new();
            println!("Name the output file [press ENTER for default]");
            io::stdin()
                .read_line(&mut filename)
                .expect("Error reading input");
            filename = filename.trim().to_string();

            if filename.is_empty() {
                filename = "decrypted_text.txt".to_string();
            } else {
                filename.push_str(".txt");
            }
            /*match custom_filename {
                Ok(file) => filename = file,
                _ => filename = "encrypted_text.txt",
            }*/
            let temp = fs::write(filename, decrypted_string.trim());
            match temp {
                Ok(_) => {
                    println!("File created successfully")
                }
                _ => {
                    println!("Error while writing the output to file:")
                }
            }
        }
    }
}
fn polybius_to_string(square: Vec<Vec<char>>) -> String {
    let headers = "ADFGVX"; // Header characters
    let c_headers = "   A D F G V X"; // Header characters
    let size = square.len();

    // Create column headers
    let column_headers: String = c_headers.chars().collect::<String>();

    // Add column headers and separator
    let mut result = format!("{:<15}   \n   {}\n", column_headers, "-".repeat(size * 2));
    dbg!("{}", &result);

    for (i, row) in square.iter().enumerate() {
        // Use the corresponding row header (A, D, F, G, V, X)
        let row_header = headers.chars().nth(i).unwrap_or(' ');
        let row_string: String = row
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" "); // Add spaces between elements
        result.push_str(&format!("{} | {}\n", row_header, row_string)); // Add row header and row data
    }

    result.to_string()
}
