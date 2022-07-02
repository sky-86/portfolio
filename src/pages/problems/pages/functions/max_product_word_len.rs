use super::super::max_product_word_len::MaxProductWordLengthsExamples;

impl MaxProductWordLengthsExamples {
    pub fn max_product(words: Vec<String>) -> i32 {
        let no_common_letters = |s1: &str, s2: &str| -> bool {
            let mut bitmask1 = 0;
            let mut bitmask2 = 0;
            for c in s1.chars() {
                bitmask1 |= 1 << (c as i32 - 'a' as i32);
            }
            for c in s2.chars() {
                bitmask2 |= 1 << (c as i32 - 'a' as i32);
            }

            (bitmask1 & bitmask2) == 0
        };

        let mut largest = 0;

        for (ie, i) in words.iter().enumerate() {
            for (je, j) in words.iter().enumerate() {
                if ie != je && no_common_letters(i, j) {
                    largest = std::cmp::max(largest, i.len() * j.len());
                }
            }
        }

        largest as i32
    }
}
