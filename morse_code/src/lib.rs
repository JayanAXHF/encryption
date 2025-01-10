use std::collections::HashMap;

pub fn encrypt_morse_code(plaintext: String) -> String {
    let words = plaintext.split_whitespace();
    let mut encrypted_string = String::new();
    let morse_code = morse_code_map();
    for (i, word) in words.enumerate() {
        if i != 0 {
            encrypted_string.push_str(" / ");
        }
        for char in word.chars() {
            let morse_letter = morse_code.get(&char.to_ascii_uppercase()).unwrap();
            encrypted_string.push_str(morse_letter);
            encrypted_string.push(' ');
        }
    }
    encrypted_string
}

pub fn decrypt_morse_code(encrypted_string: String) -> String{
    let morse_code = morse_code_map();
    let words = encrypted_string.split(" / ");
    let mut plaintext = String::new();
    for word in words {
        for char in word.split_whitespace() {
            let normal_char = morse_code
                .iter()
                .find_map(|(key, &val)| if val == char { Some(key) } else { None });
            match normal_char {
                    Some(x) => plaintext.push(*x),

                    None => println!("There is some error with the morse code map")
            }
        }
        plaintext.push(' ');
    }
   plaintext
}

fn morse_code_map() -> HashMap<char, &'static str> {
    let mut morse_map = HashMap::new();

    // Letters
    morse_map.insert('A', ".-");
    morse_map.insert('B', "-...");
    morse_map.insert('C', "-.-.");
    morse_map.insert('D', "-..");
    morse_map.insert('E', ".");
    morse_map.insert('F', "..-.");
    morse_map.insert('G', "--.");
    morse_map.insert('H', "....");
    morse_map.insert('I', "..");
    morse_map.insert('J', ".---");
    morse_map.insert('K', "-.-");
    morse_map.insert('L', ".-..");
    morse_map.insert('M', "--");
    morse_map.insert('N', "-.");
    morse_map.insert('O', "---");
    morse_map.insert('P', ".--.");
    morse_map.insert('Q', "--.-");
    morse_map.insert('R', ".-.");
    morse_map.insert('S', "...");
    morse_map.insert('T', "-");
    morse_map.insert('U', "..-");
    morse_map.insert('V', "...-");
    morse_map.insert('W', ".--");
    morse_map.insert('X', "-..-");
    morse_map.insert('Y', "-.--");
    morse_map.insert('Z', "--..");

    // Numbers
    morse_map.insert('1', ".----");
    morse_map.insert('2', "..---");
    morse_map.insert('3', "...--");
    morse_map.insert('4', "....-");
    morse_map.insert('5', ".....");
    morse_map.insert('6', "-....");
    morse_map.insert('7', "--...");
    morse_map.insert('8', "---..");
    morse_map.insert('9', "----.");
    morse_map.insert('0', "-----");

    // Punctuation
    morse_map.insert('.', ".-.-.-");
    morse_map.insert(',', "--..--");
    morse_map.insert('?', "..--..");
    morse_map.insert('\'', ".----.");
    morse_map.insert('!', "-.-.--");
    morse_map.insert('/', "-..-.");
    morse_map.insert('(', "-.--.");
    morse_map.insert(')', "-.--.-");
    morse_map.insert('&', ".-...");
    morse_map.insert(':', "---...");
    morse_map.insert(';', "-.-.-.");
    morse_map.insert('=', "-...-");
    morse_map.insert('+', ".-.-.");
    morse_map.insert('-', "-....-");
    morse_map.insert('_', "..--.-");
    morse_map.insert('"', ".-..-.");
    morse_map.insert('$', "...-..-");
    morse_map.insert('@', ".--.-.");

    morse_map
}
