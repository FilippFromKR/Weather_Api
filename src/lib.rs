#![feature(downcast_unchecked)]
mod config;
mod main;
mod error;
mod service;
mod utils;
mod dependencies;
mod storage;

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
