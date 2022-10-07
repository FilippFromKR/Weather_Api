#![feature(downcast_unchecked)]
extern crate core;
mod config;
mod error;
mod service;
pub mod utils;
mod dependencies;
mod storage;
mod weather;
pub mod input;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
