pub struct StrUtils;

impl StrUtils {

    pub fn capitalize(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        let mut chars = text.chars();
        let first_char = chars.next().unwrap().to_uppercase().to_string();
        let rest: String = chars.collect();
        
        format!("{}{}", first_char, rest)
    }

}