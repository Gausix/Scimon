pub struct StrUtils;

impl StrUtils {

    pub fn capitalize(text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        let mut chars = text.chars();
        let first_char = chars.next().unwrap().to_uppercase().to_string();
        let rest: String = chars.collect();
        
        format!("{}{}", first_char, rest)
    }

    pub fn remove_initial_character(text: &str, character: char) -> String {
        if let Some(rest) = text.strip_prefix(character) {
            return String::from(rest);
        }
        
        return String::from(text);
    }

}