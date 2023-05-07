#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

pub mod model;
pub mod parse;

#[must_use]
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
