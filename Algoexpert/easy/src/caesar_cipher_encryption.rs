struct CaesarCipherEncryption {}

impl CaesarCipherEncryption {
    fn decrypt(data: &mut String, key: i32) -> String {
        let shift = key % 26;
        let offset = 26;
        let mut target = data.clone().into_bytes();

        let len = target.len();

        let mut i = 0;

        loop {
            if i >= len {
                break;
            }
            if target[i] >= b'a' && target[i] <= b'z' {
                target[i] += shift as u8;
            } else {
                target[i] = target[i] + shift as u8 - offset;
            }
            i += 1;
        }

        target.iter_mut().map(|x| *x as char).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::caesar_cipher_encryption::CaesarCipherEncryption;

    #[test]
    fn test() {
        assert_eq!(
            CaesarCipherEncryption::decrypt(&mut "xyz".to_string(), 2),
            "zab".to_string()
        );
    }
}
