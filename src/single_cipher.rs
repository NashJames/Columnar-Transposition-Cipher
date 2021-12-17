use rand::{thread_rng, Rng};

/// Encrypts or decrypts a message, using 1 key. The encryption
/// is based on the columnar transposition method.
#[allow(dead_code)]
pub fn single_transposition(decrypt_mode: bool, msg: &str, key: &str) -> String {
    println!("Original Message: {}", msg);
    let msg: String = msg
        .to_uppercase()
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect();

    // Determines the sequence of the columns, as dictated by the
    // alphabetical order of the keyword
    let key_uppercase: String = key.to_uppercase();
    let mut key_ascii: Vec<(u8, u8)> = Vec::new();
    let mut key_order: Vec<usize> = Vec::new();
    let mut counter: u8 = 0;

    key_uppercase.bytes().for_each(|key| {
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
    let result: String = match decrypt_mode {
        false => encrypt(msg, key_order),
        true => decrypt(msg, key_order),
    };

    return result;
}

/// Performs the columnar transposition encryption
#[allow(dead_code)]
fn encrypt(mut msg: String, key_order: Vec<usize>) -> String {
    let mut encrypted_msg: String = String::from("");
    let mut encrypted_vec: Vec<String> = Vec::new();
    let mut msg_len: usize = msg.len();
    let key_len: usize = key_order.len();

    let mut key_index: usize = key_len;
    let msg_mod: usize = msg_len % key_len;

    // Loop each column, pushing it to a Vec<T>
    while !msg.is_empty() {
        let mut index: usize = 0;
        let mut chars: String = String::from("");
        key_index -= 1;

        // Loop every nth character based on key length, to create a column
        while index < msg_len {
            let ch: char = msg.remove(index);
            chars.push(ch);
            index += key_index;
            msg_len -= 1;
        }

        // If any columns are incomplete, it will fill the spare space with a random character
        if (msg_mod != 0) && (index == msg_len) {
            chars.push(thread_rng().gen_range('A'..'Z'));
        }

        encrypted_vec.push(chars + " ");
    }

    // Concatenate the columns into a string, based on the alphabetical order of the keyword's characters
    let mut indexed_vec: Vec<(usize, &String)> = Vec::new();
    let mut counter: usize = 0;

    key_order.into_iter().for_each(|index| {
        indexed_vec.push((index, &encrypted_vec[counter]));
        counter += 1;
    });

    indexed_vec.sort();

    indexed_vec.into_iter().for_each(|(_, column)| {
        encrypted_msg.push_str(column);
    });

    encrypted_msg = encrypted_msg.trim_end().to_string();

    println!("Encrypted Message: {}\n", encrypted_msg);
    return encrypted_msg;
}

/// Performs the columnar transposition decryption
#[allow(dead_code)]
fn decrypt(msg: String, key_order: Vec<usize>) -> String {
    let msg_len: usize = msg.len();
    let key_len: usize = key_order.len();

    let split_size: usize = (msg_len as f64 / key_len as f64).ceil() as usize;
    let mut split_vec: Vec<String> = Vec::new();
    let mut slice_start: usize = 0;
    let mut slice_end: usize = split_size;

    // Split the message into equal parts, based on 'message length divided by keyword length'
    while slice_end <= msg_len {
        let slice: &str = &msg[slice_start..slice_end];
        split_vec.push(slice.to_string());
        slice_start += split_size;
        slice_end += split_size;
    }

    // Concatenate the equal parts into a string, based on the
    // alphabetical order of the keyword's characters
    let mut decrypted_msg: String = String::from("");
    let mut indexed_vec: Vec<(usize, String)> = Vec::new();
    let mut sorted_vec: Vec<String> = Vec::new();
    let mut counter: usize = 0;

    key_order.into_iter().for_each(|index| {
        indexed_vec.push((counter, split_vec[index].clone()));
        counter += 1;
    });

    indexed_vec.sort();

    indexed_vec.into_iter().for_each(|(_, column)| {
        sorted_vec.push(column);
    });

    counter = 0;

    while counter != split_size {
        for index in 0..key_len {
            decrypted_msg.push(sorted_vec[index].remove(0));
        }
        counter += 1;
    }

    println!("Decrypted Message: {}\n", decrypted_msg);
    return decrypted_msg;
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn encryption() {
        assert_eq!(
            single_transposition(
                false,
                "The quick brown fox jumps over the lazy dog",
                "Archive",
            ),
            "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO"
        );

        assert!(Regex::new(
            r"DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYIQZ IRAJS[A-Z] AJSBKT HQZHR[A-Z]"
        )
        .unwrap()
        .is_match(&single_transposition(
            false,
            "abcdefghijklmnopqrstuvwxyzABCDEFGIHJKLMNOPQRSTUVWXYZ.,/;'[]{}:|_+=-`~() ",
            "Tenacious",
        )));

        assert!(
            Regex::new(r"EVLN[A-Z] ACDT[A-Z] ESEA[A-Z] ROFO[A-Z] DEEC[A-Z] WIREE")
                .unwrap()
                .is_match(&single_transposition(
                    false,
                    "WE ARE DISCOVERED. FLEE AT ONCE.",
                    "ZEBRAS",
                ))
        );
    }

    #[test]
    fn decryption() {
        assert_eq!(
            single_transposition(true, "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO", "Archive"),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"
        );

        assert_eq!(
            single_transposition(
                true,
                "DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYIQZ IRAJSW AJSBKT HQZHRI",
                "Tenacious",
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGIHJKLMNOPQRSTUVWXYZIW"
        );

        assert_eq!(
            single_transposition(true, "EVLNE ACDTK ESEAQ ROFOJ DEECU WIREE", "ZEBRAS"),
            "WEAREDISCOVEREDFLEEATONCEQKJEU"
        );
    }
}
