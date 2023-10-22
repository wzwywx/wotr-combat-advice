//! # wotr-combat-advice
//!
//! Utilities to help win combats in Pathfinder: Wrath of the Righteous (WOTR).
//!
//! ### Current Advice Available
//! 1. What is the probability of X hitting Y? This can be used for the general case of roll + modifiers VS difficulty check.

/// Finds the probability of succeeding against a difficulty value. Note that the difficulty value can be any value that the modifiers can be used against ,e.g. DC and AC,
/// since mechanically they all work the same way even if the context is different.
///
/// Order of the first two arguments is important here. The first argument should be the modifiers of the challenger (the person or situation presented with the difficulty)
/// while the second argument should be the difficulty check. Mixing up the two might give you the wrong probability.
///
/// **Example**
/// ```
/// let success_chance = find_success_prob_against_difficulty(10, 20, 20);
/// assert_eq!(success_chance, 0.8);
/// ```
pub fn find_success_prob_against_difficulty(
    challenger_modifier: u32,
    difficulty_check: u32,
    dice_size: u32,
) -> f32 {
    todo!("currently implementing");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = find_success_prob_against_dc(10, 20, 20);
        assert_eq!(result, 0.45);
    }
}
