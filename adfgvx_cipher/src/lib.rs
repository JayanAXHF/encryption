use std::collections::HashMap;

use utils::*;

pub fn encrypt_adfgvx(plaintext: String, keyword: String, column_key: Vec<u8>) -> String {
    let adfgvx = ['A', 'D', 'F', 'G', 'V', 'X'];
    let polybius_square = polybius_square(keyword);
    let mut intermediate = String::new();
    for letter in plaintext.chars() {
        if letter.is_whitespace() {
            continue;
        }
        let (col, row) = find_char(&polybius_square, letter.to_ascii_uppercase()).unwrap();
        intermediate.push_str(&format!("{}{}", adfgvx[row], adfgvx[col]));
    }
    intermediate = remove_whitespace(&mut intermediate);
    let slice = &intermediate.chars().collect::<Vec<_>>()[..];
    let mut chunked = slice
        .chunks(6)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();
    if chunked.last().unwrap().len() != 6 {
        for _ in 0..(6 - chunked.last().unwrap().len()) {
            let len = chunked.len() - 1;
            chunked[len].push('X');
            chunked[len].push('X');
        }
    }
    //let slice2 = &chunked.into_iter().flatten().collect::<Vec<char>>()[..];
    //let mut chunked = slice2.chunks(len_chunked).map(|chunk|chunk.to_vec()).collect::<Vec<_>>();
    let transposed_matrix = transpose(chunked.clone());
    let mut order: Vec<u8> = Vec::new();
    for i in 0..6 {
        let index = column_key.iter().position(|&r| r == i + 1).unwrap();
        order.push(index.try_into().unwrap());
    }
    let mut ciphertext: String = String::new();
    for i in order {
        let string = transposed_matrix[i as usize].iter().collect::<String>();
        ciphertext.push_str(&string);
    }
    ciphertext
}

pub fn polybius_square(keyword: String) -> Vec<Vec<char>> {
    let non_repeating_string = remove_repeating_letters(keyword).to_ascii_uppercase();
    let mut square: Vec<Vec<char>> = Vec::new();
    let charset = remove_charset(
        non_repeating_string.chars().collect(),
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
    );
    let combined = format!("{non_repeating_string}{charset}");
    let charset = &combined.chars().collect::<Vec<_>>()[..];
    let mut charset = charset.chunks(6).map(|chunk| chunk.to_vec()).collect();
    square.append(&mut charset);

    square
}

pub fn decrypt_adfgvx(encrypted_text: String, key: String, column_key: Vec<u8>) -> String {
    let length = encrypted_text.len();
    let slice = &encrypted_text.chars().collect::<Vec<char>>()[..];
    let mut encrypted_text = encrypted_text.clone();
    if length%6 != 0{
        for _ in 0..(length%6){
            encrypted_text.push_str("XX");
        }
    }
    let chunked = slice
        .chunks(length / 6_usize)
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
    let order: Vec<u8> = column_key.iter().map(|key| (key - 1)).collect();
    let mut intermediate = String::new();
    let mut intermediate_vector = Vec::new();
    for i in order {
        let chunk = &chunked[i as usize];
        intermediate_vector.push(chunk.clone());
    }
    let transposed_matrix = transpose(intermediate_vector);
    for vector in transposed_matrix {
        let string = vector.iter().collect::<String>();
        intermediate.push_str(&string);
    }
    let mut plaintext = String::new();
    let square = polybius_square(key);
    let letter_indices =
        HashMap::from([('A', 0), ('D', 1), ('F', 2), ('G', 3), ('V', 4), ('X', 5)]);
    let intermediate_sliced = &intermediate.chars().collect::<Vec<_>>()[..];
    let intermediate_pairs = intermediate_sliced
        .chunks(2)
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
    for pair in intermediate_pairs {
        let row_index = letter_indices.get(&pair[0]).unwrap();
        let column_index = letter_indices.get(&pair[1]).unwrap();
        let letter = square[*row_index as usize][*column_index as usize];
        plaintext.push(letter);
    }
    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polybius_square() {
        let key = remove_whitespace(&mut String::from("lorem ipsum sir"));
        println!("{:?}", polybius_square(key.clone()));
        assert_eq!(
            "loremi".to_ascii_uppercase().chars().collect::<Vec<char>>(),
            polybius_square(key)[0]
        );
    }
    #[test]
    fn test_encryptor() {
        let keyword = String::from("aarav");
        let plaintext = String::from("certified loverboy");
        let column_key: Vec<u8> = vec![5, 1, 3, 4, 2, 6];
        assert_eq!("", encrypt_adfgvx(plaintext, keyword, column_key))
    }
}
