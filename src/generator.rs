use std::io;
use std::fs::File;
use std::io::Read;

use error::*;


const NBITS:usize = 256;

pub fn blum_blum_shub(iterations: usize) -> Result<usize> {
    let p = random_prime(NBITS)?;
    let q = random_prime(NBITS)?;
    let m = p * q;
    Ok((0..iterations).fold(2, |acc, _| (acc * acc) % m))
}

fn random_prime(nbits: usize) -> Result<usize>{
    let mut buff = vec![0u8; nbits];
    let _ = generate_entropy(&mut buff[..]);
    let limit =  buff.iter().map(|x| *x as usize).sum();
    create_large_prime(limit)
}

fn create_large_prime(limit: usize) -> Result<usize>{
    // Using Sieve of Eratosthenes. Maybe another implementation?
    // FIX: non idiomatic
    if limit < 3 { return Ok(limit);}
    let mut primes: Vec<usize> = Vec::with_capacity(limit / 10);
    // base case
    primes.push(2);
    for el in 3..limit + 1{
        let mut composite = false;
        for p in &mut primes{
            if el % *p == 0 {
                composite = true;
                break;
            }
        }
        if !composite {primes.push(el);}
    }
    if let Some(result) = primes.iter().last() {
        return Ok(*result);
    }
   Ok(1)
}

fn generate_entropy(buff: &mut [u8]) -> Result<()> {
    // Only *nix
    let urand = File::open("/dev/urandom")?;
    let mut reader = io::BufReader::new(urand);
    reader.read_exact(&mut buff[..])?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_random_prime(){
        assert!(random_prime(100).unwrap() > 1);
    }

    #[test]
    fn test_create_prime(){
        assert_eq!(1, create_large_prime(1).unwrap());
        assert_eq!(1021, create_large_prime(1024).unwrap());
        assert_eq!(99991, create_large_prime(100_000).unwrap());
    }

    #[test]
    fn test_generate_entroyp(){
        let nbits = 10_0000;
        let mut buff = vec![0u8; nbits];
        generate_entropy(&mut buff[..]).unwrap();
        assert_eq!(buff.len(), nbits);
    }

}
