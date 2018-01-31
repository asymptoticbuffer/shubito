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
    fn test_blum_blum_shub(){
        assert!(generator::blum_blum_shub(10_000).unwrap() > 10);
    }
}
