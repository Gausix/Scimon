use rand::Rng;

pub struct FileName {
    length: usize,
    pattern: String,
}

impl FileName {

    pub fn new(length: usize) -> Self {
        Self {
            length,
            pattern: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        }
    }
    
    pub fn gen(&self) -> String {
        let charset_len: usize = self.pattern.len();
        let mut rng = rand::thread_rng();
    
        let random_string: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset_len);
                self.pattern.chars().nth(idx).unwrap()
            })
            .collect();
    
        format!(
            "{}.html", random_string
        )
    }

}
