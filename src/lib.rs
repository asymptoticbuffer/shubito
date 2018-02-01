#![deny(warnings)]

#[macro_use]
extern crate error_chain;

pub mod error{
	error_chain! {
		types {
			Error, ErrorKind, ResultExt, Result;
		}

		foreign_links {
			Fmt(::std::fmt::Error);
			Io(::std::io::Error);
		}
	}
}

pub mod generator;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_random_numb(){
        assert!(generator::generate_random_number().unwrap() > 10);
    }
}
