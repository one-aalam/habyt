
//! # Habyt
//!
//! `lib` is a collection of re-usable `habyt` code
//!
/// Adds one to the provided number. ///
/// # Examples
///
/// ```
/// let two = 2;
///
/// assert_eq!(lib::add_one(two), 3);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(2), 3);
    }
}
