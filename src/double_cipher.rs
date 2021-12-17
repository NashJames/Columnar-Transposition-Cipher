// use std::io::BufRead;

// use rand::{thread_rng, Rng};
// use regex::Regex;

/// Encrypts or decrypts a message twice, using 1 or 2 keys. The
/// encryption is based on the columnar transposition method.
pub fn double_transposition(decrypt_mode: bool, msg: &str, key: &str) -> String {
    let key_uppercase: String = key.to_uppercase();
    let mut cipher_msg: String = msg.to_string();
    // let mut key_num: usize = 0;

    let key_split_vec: Vec<&str> = match decrypt_mode {
        false => key_uppercase.split_whitespace().collect(),
        true => key_uppercase.split_whitespace().rev().collect(),
    };

    for key_split in &key_split_vec {
        // key_num += 1;
        // let final_key: bool = key_num == key_split_vec.len();

        // Determines the sequence of the columns, as dictated by the
        // alphabetical order of the keyword
        let mut key_ascii: Vec<(u8, u8)> = Vec::new();
        let mut key_order: Vec<usize> = Vec::new();
        let mut counter: u8 = 0;

        cipher_msg = cipher_msg
            .to_uppercase()
            .chars()
            .filter(|&c| c.is_ascii_alphabetic())
            .collect();

        key_split.bytes().for_each(|key| {
            key_ascii.push((key, counter));
            counter += 1;
        });

        key_ascii.sort();
        let mut counter: u8 = 0;

        key_ascii.iter_mut().for_each(|(key, _)| {
            *key = counter;
            counter += 1;
        });

        key_ascii.sort_by_key(|&(_, index)| index);

        key_ascii
            .into_iter()
            .for_each(|(key, _)| key_order.push(key.into()));

        // Determines whether to encrypt or decrypt the message,
        // and returns the result
        cipher_msg = match decrypt_mode {
            false => encrypt(cipher_msg, key_order),
            true => decrypt(cipher_msg, key_order),
        };
    }

    println!("Original Message: {}", msg);
    match decrypt_mode {
        false => println!("Encrypted Message: {}\n", cipher_msg),
        true => println!("Decrypted Message: {}\n", cipher_msg),
    };

    return cipher_msg;
}

/// Performs the columnar transposition encryption
fn encrypt(mut msg: String, key_order: Vec<usize>) -> String {
    let mut encrypted_msg: String = String::from("");
    let mut encrypted_vec: Vec<String> = Vec::new();
    // let mut chars: String = String::from("");
    let msg_len: usize = msg.len();
    let key_len: usize = key_order.len();

    // println!("{:?}", key_order);
    // println!("{}", msg);
    // println!("{}", msg_len);

    let mut msg_index: usize = msg_len;
    let mut key_index: usize = key_len;
    // let msg_mod: bool = (msg_len % key_len) != 0;

    // Loop each column, pushing it to a Vec<T>
    while !msg.is_empty() {
        let mut index: usize = 0;
        let mut chars: String = String::from("");
        key_index -= 1;

        // Loop every nth character based on key length, to create a column
        while index < msg_index {
            let ch: char = msg.remove(index);
            chars.push(ch);
            index += key_index;
            msg_index -= 1;
        }

        // If any columns are incomplete, it will fill the spare space with a random character
        // if (msg_mod) && (index == msg_len) {
        //     if final_key {
        //         chars.push(thread_rng().gen_range('A'..'Z'));
        //     }
        //     // else {
        //     //     chars.push(' ');
        //     // }
        // }

        encrypted_vec.push(chars);
    }

    println!("Encrypted Vec: {:?}", encrypted_vec);

    // Concatenate the columns into a string, based on the alphabetical order of the keyword's characters
    let mut indexed_vec: Vec<(usize, &String)> = Vec::new();
    let mut indexed_msg: String = String::from("");
    let mut counter: usize = 0;

    key_order.into_iter().for_each(|index| {
        indexed_vec.push((index, &encrypted_vec[counter]));
        counter += 1;
    });

    indexed_vec.sort();

    println!("{:?}", indexed_vec);

    indexed_vec.into_iter().for_each(|(_, column)| {
        indexed_msg.push_str(column);
    });

    // Split the message by a space every nth character, determined by (msg_len / key_len).ceil()
    let msg_div: i32 = (msg_len as f32 / key_len as f32).ceil() as i32;
    let mut counter: i32 = 0;

    indexed_msg.chars().for_each(|c| {
        counter += 1;
        encrypted_msg.push(c);
        if counter == msg_div {
            counter = 0;
            encrypted_msg.push(' ');
        }
    });

    encrypted_msg = encrypted_msg.trim_end().to_string();

    return encrypted_msg;
}

/// Performs the columnar transposition decryption
fn decrypt(mut msg: String, key_order: Vec<usize>) -> String {
    let msg_len: usize = msg.len();
    let key_len: usize = key_order.len();

    // println!("{:?}", key_order);
    // println!("{}", msg);
    // println!("{}", msg_len);
    // println!("{}", (msg_len as f64 / key_len as f64));

    let split_size: usize = (msg_len as f64 / key_len as f64) as usize;

    // if final_key {
    //     split_size += 1;
    // }
    // let mut split_vec: Vec<String> = Vec::new();
    let mut split_vec: Vec<(usize, String)> = Vec::new();
    // let mut slice_start: usize = 0;
    // let mut slice_end: usize = split_size;
    let mut counter: usize = 0;

    let msg_mod = msg_len % key_len;
    let mut key_split = key_order.clone();
    // let mut split_large = key_order.clone();
    let (split_large, split_small) = key_split.split_at_mut(msg_mod);
    // split_large.truncate(msg_mod);
    split_large.sort();
    split_small.sort();

    println!("Split large: {:?}", split_large);
    println!("Split small: {:?}", split_small);

    split_large.into_iter().for_each(|key| {
        // println!(
        //     "{}..{}",
        //     (*key * split_size),
        //     (((*key + 1) * split_size) + counter)
        // );
        counter += 1;
        let slice: &str =
            &msg[((*key * split_size) + counter - 1)..(((*key + 1) * split_size) + counter)];
        println!("Slice: {}", slice);
        split_vec.push((*key, slice.to_string()));
        // msg.replace_range((*key * split_size)..(((*key + 1) * split_size) + 1), "");
    });

    split_large.into_iter().rev().for_each(|key| {
        counter -= 1;
        // println!(
        //     "{}..{}",
        //     ((*key * split_size) + counter - 1),
        //     (((*key + 1) * split_size) + counter)
        // );
        let slice: &str =
            &msg[((*key * split_size) + counter)..(((*key + 1) * split_size) + counter + 1)];
        println!("Slice 2: {}", slice);
        msg.replace_range(
            ((*key * split_size) + counter)..(((*key + 1) * split_size) + counter + 1),
            "",
        );
    });

    // Encrypted Vec: ["AJSBKT", "BKTCLU", "CLUDMV", "DMVENW", "ENWFOX", "FOXGPY", "GPYHQZ", "HQZIR", "IRAJS"]
    println!("Msg: {}", msg);

    split_small.into_iter().for_each(|key| {
        let (slice, rest_of_msg) = msg.split_at(split_size);
        split_vec.push((*key, (slice.to_string())));
        msg = rest_of_msg.to_string();
        // slice_start += split_size;
        // slice_end += split_size;
        // counter += 1;

        // let slice: &str = &msg[(*key * split_size)..((*key + 1) * split_size)];
        // println!("Slice: {}", slice);
        // split_vec.push((*key, slice.to_string()));
        // // msg.replace_range((*key * split_size)..((*key + 1) * split_size), "");
        // counter += 1;
    });

    // Split the message into equal parts, based on 'message length divided by keyword length'
    // while slice_end <= (msg_len - (msg_mod * (split_size + 1))) {
    //     // let slice: &str = &msg[slice_start..slice_end];
    //     let (slice, rest_of_msg) = msg.split_at(split_size);
    //     split_vec.push(counter, (slice.to_string()));
    //     msg = rest_of_msg.to_string();
    //     // slice_start += split_size;
    //     slice_end += split_size;
    //     counter += 1;
    // }

    println!("Split vec: {:?}", split_vec);

    split_vec.sort();

    println!("Split vec: {:?}", split_vec);

    // Concatenate the equal parts into a string, based on the
    // alphabetical order of the keyword's characters
    let mut decrypted_msg: String = String::from("");
    // let mut indexed_vec: Vec<(usize, String)> = Vec::new();
    let mut sorted_vec: Vec<String> = Vec::new();
    // let mut counter: usize = 0;

    // println!("Key order: {:?}", key_order);

    // key_order.into_iter().for_each(|index| {
    //     // println!("{}: {}", index, split_vec[index].clone());
    //     indexed_vec.push((counter, split_vec[index].clone()));
    //     counter += 1;
    // });

    // println!("Indexed: {:?}", indexed_vec);

    // // indexed_vec.sort();

    // println!("Sorted: {:?}", indexed_vec);

    key_order.into_iter().for_each(|key| {
        if let Some((_, column)) = split_vec.iter().find(|(a, _)| a == &key) {
            sorted_vec.push(column.to_string());
        }
    });

    // split_vec.into_iter().for_each(|(_, column)| {
    //     sorted_vec.push(column);
    // });

    // key_order.into_iter().for_each(|index| {
    //     sorted_vec.push(split_vec[index].clone());
    // });

    println!("Sorted Vec: {:?}", sorted_vec);

    // for i in key_len..split_vec.len() {
    //     sorted_vec.push(split_vec[i].clone());
    // }

    // if !msg.is_empty() {
    //     let mut counter: usize = 0;
    //     msg.chars().for_each(|c| {
    //         sorted_vec[counter].push(c);
    //         counter += 1;
    //     })
    // }

    // println!("{:?}", sorted_vec);
    let mut counter = 0;

    while counter != split_size {
        for index in 0..key_len {
            decrypted_msg.push(sorted_vec[index].remove(0));
        }
        counter += 1;
    }

    if !sorted_vec.is_empty() {
        sorted_vec.into_iter().for_each(|c| {
            decrypted_msg.push_str(&c);
        })
    }

    println!("Decrypted: {}", decrypted_msg);

    return decrypted_msg;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption() {
        assert_eq!(
            double_transposition(
                false,
                "The quick brown fox jumps over the lazy dog",
                "Archive",
            ),
            "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO"
        );

        assert_eq!(
            double_transposition(
                false,
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.,/;'[]{}:|_+=-`~() ",
                "Tenacious"
            ),
            "DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYHQZ IRAJSA JSBKTH QZIR"
        );

        assert_eq!(
            double_transposition(false, "WE ARE DISCOVERED. FLEE AT ONCE.", "ZEBRAS"),
            "EVLNA CDTES EAROF ODEEC WIREE"
        );
    }

    #[test]
    fn decryption() {
        assert_eq!(
            double_transposition(true, "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO", "Archive"),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"
        );

        assert_eq!(
            double_transposition(
                true,
                "DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYHQZ IRAJSA JSBKTH QZIR",
                "Tenacious"
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        assert_eq!(
            double_transposition(true, "EVLNA CDTES EAROF ODEEC WIREE", "ZEBRAS"),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }

    #[test]
    fn double_encryption() {
        assert_eq!(
            double_transposition(
                false,
                "The quick brown fox jumps over the lazy dog",
                "Archive Snow"
            ),
            "KEZEUWHAH ORCGRMBIO TLESOUDVP OJFQYTXN"
        );

        assert_eq!(
            double_transposition(
                false,
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.,/;'[]{}:|_+=-`~() ",
                "Tenacious Drink"
            ),
            "DWOCXLGZSKI VNBUPDYRJHN FTOCVQJBZEW KFYMHASQMEX LGUPIATR"
        );

        assert_eq!(
            double_transposition(false, "WE ARE DISCOVERED. FLEE AT ONCE.", "ZEBRAS STRIPE"),
            "CAEEN SOIAE DRLEF WEDRE EVTOC"
        );
    }

    #[test]
    fn double_decryption() {
        assert_eq!(
            double_transposition(
                true,
                "KEZEUWHAH ORCGRMBIO TLESOUDVP OJFQYTXN",
                "Archive Snow"
            ),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"
        );

        assert_eq!(
            double_transposition(
                true,
                "DWOCXLGZSKI VNBUPDYRJHN FTOCVQJBZEW KFYMHASQMEX LGUPIATR",
                "Tenacious Drink",
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        assert_eq!(
            double_transposition(true, "CAEEN SOIAE DRLEF WEDRE EVTOC", "ZEBRAS STRIPE"),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }
}
