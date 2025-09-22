fn is_vowel(c: char) -> bool { 
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn convert_word_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    if !first_char.is_ascii_alphabetic() {
        word.to_string()
    } else if is_vowel(first_char) {
        format!("{}-hay", word)
    } else { 
        if word.len() == 1 { 
            format!("{}-ay", word)
        } else {
            format!("{}-{}ay", &word[1..], first_char)
        }
    }
}

pub fn to_pig_latin(input: &str) -> String {
    
    if input.trim().is_empty() {
        input.to_string()
    }else {
        input.split_whitespace()
            .map(convert_word_to_pig_latin)
            .collect::<Vec<String>>()
            .join(" ")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pig_latin_one_word_consonant() {
        let result = to_pig_latin("first");
        assert_eq!(result, "irst-fay");
    }

    #[test]
    fn test_pig_latin_one_word_vowel() {
        let result = to_pig_latin("apple");
        assert_eq!(result, "apple-hay");
    }

    #[test]
    fn test_pig_latin_multi_word() {
        let result = to_pig_latin("hello world");
        assert_eq!(result, "ello-hay orld-way");
        let result = to_pig_latin("eat pie");
        assert_eq!(result, "eat-hay ie-pay");
    }

    #[test]
    fn test_pig_latin_empty_string() {
        let result = to_pig_latin("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_pig_latin_single_char() {
        let result = to_pig_latin("a");
        assert_eq!(result, "a-hay");
        let result = to_pig_latin("b");
        assert_eq!(result, "b-ay");
    }

    #[test]
    fn test_pig_latin_non_alpha_start() {
        let result = to_pig_latin("1st place");
        assert_eq!(result, "1st lace-pay");
        let result = to_pig_latin("#hashtag");
        assert_eq!(result, "#hashtag");
        let result = to_pig_latin("!exclaim apple");
        assert_eq!(result, "!exclaim apple-hay");
    }

}