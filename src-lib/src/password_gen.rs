use super::{PassGen, PasswordGen, PassGenError};
use rand::seq::SliceRandom;

impl PassGen<char> for PasswordGen {
    fn new(length: usize, tokenset: Vec<char>, _separator: Option<char>, _word_case: Option<bool>) -> Result<Self, PassGenError> {
        if length == 0 {
            return Err(PassGenError::InvalidLength);
        }
        if tokenset.is_empty() {
            return Err(PassGenError::EmptyTokenSet);
        }
        Ok(Self { length, tokenset })
    }

    fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        (0..self.length)
            .map(|_| *self.tokenset.choose(&mut rng).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_gen_new() {
        let tokenset = vec!['a', 'b', 'c', 'd', 'e'];
        
        // Test valid input
        let password_gen = PasswordGen::new(8, tokenset.clone(), None, None);
        assert!(password_gen.is_ok());
        
        // Test invalid length
        let password_gen = PasswordGen::new(0, tokenset.clone(), None, None);
        assert_eq!(password_gen.unwrap_err(), PassGenError::InvalidLength);
        
        // Test empty tokenset
        let password_gen = PasswordGen::new(8, vec![], None, None);
        assert_eq!(password_gen.unwrap_err(), PassGenError::EmptyTokenSet);
    }

    #[test]
    fn test_password_gen_generate() {
        let tokenset = vec!['a', 'b', 'c', 'd', 'e'];
        
        // Test password generation
        let password_gen = PasswordGen::new(8, tokenset.clone(), None, None).unwrap();
        let password = password_gen.generate();
        assert_eq!(password.len(), 8);
        assert!(password.chars().all(|c| tokenset.contains(&c)));
    }
}