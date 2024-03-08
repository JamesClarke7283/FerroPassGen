use super::{PassGen, PassphraseGen, PassGenError};
use rand::seq::SliceRandom;

impl PassGen<String> for PassphraseGen {
    fn new(
        length: usize,
        tokenset: Vec<String>,
        separator: Option<char>,
        word_case: Option<bool>,
    ) -> Result<Self, PassGenError> {
        if length == 0 {
            return Err(PassGenError::InvalidLength);
        }
        if tokenset.is_empty() {
            return Err(PassGenError::EmptyTokenSet);
        }
        Ok(Self {
            length,
            tokenset,
            separator: separator.unwrap_or('-'),
            word_case: word_case.unwrap_or(false),
        })
    }

    fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let words: Vec<String> = (0..self.length)
            .map(|_| {
                let word = self.tokenset.choose(&mut rng).unwrap();
                if self.word_case {
                    word.to_uppercase()
                } else {
                    word.to_lowercase()
                }
            })
            .collect();
        words.join(&self.separator.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passphrase_gen_new() {
        let tokenset = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
        
        // Test valid input
        let passphrase_gen = PassphraseGen::new(4, tokenset.clone(), None, None);
        assert!(passphrase_gen.is_ok());
        
        // Test invalid length
        let passphrase_gen = PassphraseGen::new(0, tokenset.clone(), None, None);
        assert_eq!(passphrase_gen.unwrap_err(), PassGenError::InvalidLength);
        
        // Test empty tokenset
        let passphrase_gen = PassphraseGen::new(4, vec![], None, None);
        assert_eq!(passphrase_gen.unwrap_err(), PassGenError::EmptyTokenSet);
    }

    #[test]
    fn test_passphrase_gen_generate() {
        let tokenset = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
        
        // Test default separator and word case
        let passphrase_gen = PassphraseGen::new(4, tokenset.clone(), None, None).unwrap();
        let passphrase = passphrase_gen.generate();
        assert_eq!(passphrase.split('-').count(), 4);
        assert!(passphrase.chars().all(|c| c.is_lowercase() || c == '-'));
        
        // Test custom separator
        let passphrase_gen = PassphraseGen::new(4, tokenset.clone(), Some('_'), None).unwrap();
        let passphrase = passphrase_gen.generate();
        assert_eq!(passphrase.split('_').count(), 4);
        
        // Test uppercase word case
        let passphrase_gen = PassphraseGen::new(4, tokenset, None, Some(true)).unwrap();
        let passphrase = passphrase_gen.generate();
        assert!(passphrase.chars().all(|c| c.is_uppercase() || c == '-'));
    }
}