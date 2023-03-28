struct Solution {}

impl Solution {
    pub fn valid_word_square(words: Vec<String>) -> bool {
        for i in 0..words.len() {
            for j in 0..words[i].len() {
                if i >= words.len() || i >= words[j].len() || words[i].as_bytes()[j] != words[j].as_bytes()[i] {
                    return false;
                }
            }
        }

        return true;
    }
}
