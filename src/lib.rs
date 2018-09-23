#![feature(duration_as_u128)]
#![feature(test)]
extern crate rand;
extern crate test;

use rand::rngs::SmallRng;
use rand::{SeedableRng, RngCore};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Seed {}
pub struct Prng {}
pub enum SeedType {
    Nanos,
    AllocAddr
}

impl Seed {
     
    pub fn with_alloc_addr() -> [u8; 16] {
        //has entropy for first call only.
        let seed = &false as *const _ as u128;
        let seed_bytes: [u8; 16] = Self::transmute(seed);
        //println!("ADDR_SEED {:?}", seed_bytes);   
        seed_bytes
     }

    pub fn with_nanos() -> [u8; 16] {    
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();    
        let nanos: u128 = since_the_epoch.as_nanos();     
        let seed_bytes: [u8; 16] = Self::transmute(nanos);
        //println!("TICK_SEED {:?}", seed_bytes);   
        seed_bytes
    }

    fn transmute(x: u128) -> [u8; 16] {

        let b1 : u8 = ((x >> 120) & 0xffffffff) as u8;
        let b2 : u8 = ((x >> 112) & 0xffffffff) as u8;
        let b3 : u8 = ((x >> 104) & 0xffffffff) as u8;
        let b4 : u8 = ((x >> 96) & 0xffffffff) as u8;    
        
        let b5 : u8 = ((x >> 88) & 0xffffffff) as u8;
        let b6 : u8 = ((x >> 80) & 0xffffffff) as u8;
        let b7 : u8 = ((x >> 72) & 0xffffffff) as u8;    
        let b8 : u8 = ((x >> 64) & 0xffffffff) as u8;    

        let b9 : u8 = ((x >> 56) & 0xffffffff) as u8;
        let b10 : u8 = ((x >> 48) & 0xffffffff) as u8;
        let b11 : u8 = ((x >> 40) & 0xffffffff) as u8;
        let b12 : u8 = ((x >> 32) & 0xffffffff) as u8;
        
        let b13 : u8 = ((x >> 24) & 0xffffffff) as u8;
        let b14 : u8 = ((x >> 16) & 0xffffffff) as u8;
        let b15 : u8 = ((x >> 8) & 0xffffffff) as u8;    
        let b16 : u8 = (x & 0xffffffff) as u8;

        //Most of the entropy is in the last few bytes and generators are allowed
        //to assume evenly spread entropy in the seed, so spread the bytes around
        [b16, b1, b14, b3, b12, b5, b10, b7, b8, b9, b6, b11, b4, b13, b2, b15]
    }
}

impl Prng {    

    pub fn new() -> Box<RngCore> {
        Self::with(SeedType::Nanos)
    }

    pub fn with(seed_type: SeedType) -> Box<RngCore> {
         let seed_bytes: [u8; 16];
         match seed_type {
             SeedType::Nanos => seed_bytes = Seed::with_nanos(),
             SeedType::AllocAddr => seed_bytes = Seed::with_alloc_addr()
         }
        let prng = SmallRng::from_seed(seed_bytes);
        Box::new(prng)
    }        
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Range, Distribution};
    //use test::Bencher;

    const TEST_ITER: usize = 20;

    //todo: measure 0s and 1s in seed 
    //todo: measure the quality of the distribution

    #[test] 
    fn next_u32_nanos() {
        let mut prng = Prng::with(SeedType::Nanos);
        for _ in 0..TEST_ITER {
            println!("{} ", prng.next_u32());
        }
    }

     #[test]
    fn next_u32_addr() {
        let mut prng = Prng::with(SeedType::AllocAddr);
        for _ in 0..TEST_ITER {
            println!("{} ", prng.next_u32());
        }
    }
 
     #[test]
    fn range_sample_ticks() {
        let mut prng = Prng::new();        
        let range = Range::new(0, 100);
        for _ in 0..TEST_ITER {
            println!("{} ", range.sample(&mut prng));
        }
    }
}