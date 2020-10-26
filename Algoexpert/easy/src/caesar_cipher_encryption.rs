use std::borrow::BorrowMut;

struct CaesarCipherEncryption {}

impl CaesarCipherEncryption {
    fn decrypt(data: &mut String, key: i32) -> String {
        let shift = key % 26;
        let offset = 26;
        let mut target = data.borrow_mut().as_bytes();

        for i in 0..target.len() {
            let mut a = b"";
            if target[i] >= b'a' && target[i] <= b'z' {
                target[i] += 2 as u8;
            } else {}
        }


        "fasd".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::caesar_cipher_encryption::CaesarCipherEncryption;

    #[test]
    fn test() {
        assert_eq!(CaesarCipherEncryption::decrypt(&mut "xyz".to_string(), 2), "zab".to_string());
    }
}