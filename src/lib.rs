#![deny(warnings)]

#[macro_use]
extern crate error_chain;

pub mod error;
pub mod generator;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_random_numb(){
        assert!(generator::generate_random_number().unwrap() > 10);
    }

    #[test]
    fn test_generate_random_prime(){
        assert!(generator::generate_random_prime().unwrap() > 10);
    }

}
