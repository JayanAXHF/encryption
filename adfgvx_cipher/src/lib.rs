use utils::*;

pub fn encrypt_adfgvx(plaintext: String, keyword: String, column_key: Vec<u8>) -> String {
    let adfgvx = ['A', 'D', 'F', 'G', 'V', 'X'];
    let polybius_square = polybius_square(keyword);
    dbg!("{:#?}", &polybius_square);
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
    let len_chunked = chunked.len();
    if chunked.last().unwrap().len() != 6 {
        for _ in 0..(6 - chunked.last().unwrap().len()) {
            let len = chunked.len() - 1;
            //chunked[len].push('X');
            //chunked[len].push('X');
        }
    }
    println!("{:?}", intermediate);
    //let slice2 = &chunked.into_iter().flatten().collect::<Vec<char>>()[..];
    //let mut chunked = slice2.chunks(len_chunked).map(|chunk|chunk.to_vec()).collect::<Vec<_>>();

    let transposed_matrix = transpose(chunked.clone());
    println!("{:#?}\n{:#?}", chunked, transposed_matrix);
    println!("{:#?}", column_key);
    let mut order: Vec<u8> = Vec::new();
    for i in 0..6 {
        let index = column_key.iter().position(|&r| r == i + 1).unwrap();
        order.push(index.try_into().unwrap());
    }
    println!("{:#?}", order);
    let mut ciphertext: String = String::new();
    for i in order {
        let string = transposed_matrix[i as usize].iter().collect::<String>();
        ciphertext.push_str(&string);
        println!("------\n{}", i)
    }
    ciphertext
}

fn polybius_square(keyword: String) -> Vec<Vec<char>> {
    let non_repeating_string = remove_repeating_letters(keyword.clone()).to_ascii_uppercase();
    let mut square: Vec<Vec<char>> = Vec::new();
    let charset = remove_charset(
        non_repeating_string.chars().collect(),
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
    );
    let combined = format!("{non_repeating_string}{charset}");
    let charset =&combined
        .chars()
        .collect::<Vec<_>>()[..];
    let mut charset = charset.chunks(6).map(|chunk| chunk.to_vec()).collect();
    square.append(&mut charset);

    square
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
