use std::io;

fn main() {
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the
    // word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay
    // added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8
    // encoding!
    println!("Enter a word:");
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    println!("Pig-latin: {}", pig_latin(&word.to_string()));
}

fn pig_latin(s: &String) -> String {
    let s = s.trim();
    let chars: Vec<char> = s.chars().collect();
    let vowels = String::from("aeiou");

    match chars.first() {
        Some(c) if vowels.contains(*c) => format!("{}-hay", s),
        Some(c) => format!("{}-{}ay", &s[1..], c),
        _ => s.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pig_latin_first() {
        let input = "first".to_string();
        let output = pig_latin(&input);
        assert_eq!(output, "irst-fay");
    }

    #[test]
    fn test_pig_latin_apple() {
        let input = "apple".to_string();
        let output = pig_latin(&input);
        assert_eq!(output, "apple-hay");
    }

    #[test]
    fn test_pig_latin_empty() {
        let input = "".to_string();
        let output = pig_latin(&input);
        assert_eq!(output, "");
    }

    #[test]
    fn test_pig_latin_newline() {
        let input = "test\n".to_string();
        let output = pig_latin(&input);
        assert_eq!(output, "est-tay");
    }
}