//! # wotr-utils
//!
//! Utilities to help win combats in Pathfinder: Wrath of the Righteous (WOTR).
//!
//! ## Current Advice
//! 1. What is the probability of X hitting Y? This can be used for the  general case of roll + modifiers VS difficulty check.

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
