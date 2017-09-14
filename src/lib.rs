//! Fisher's exact test from the [R Project](https://www.r-project.org/) in a Rust crate.

extern "C" {
    fn r_fishers_exact(table: *mut i32, tails: i32) -> f64;
}

/// Passed to the fishers_exact function to express whether to perform a
/// one or two tailed statistical test.
#[derive(Clone, Copy)]
pub enum TestTails {
    One,
    Two
}

/// Computes the [Fisher's exact test](https://en.wikipedia.org/wiki/Fisher%27s_exact_test)
/// to determine if there are nonrandom associations between two
/// categorical variables, in a two by two contingency table.
///
/// The test is computed using the Fisher's exact test code copied from
/// the [R statistical computing package](https://www.r-project.org/).
///
/// Given a one dimensional representation of a two by two contingency,
/// table [a,b,c,d], the test computes:
///
/// p = ((a + b)!(c + d)!(a + c)!(b + d)!) / (a! b! c! d! (a+b+c+d)!)
///
/// Either a one tailed or two tailed test can be computed.
///
/// ```
/// use fishers_exact::{fishers_exact,TestTails};
///
/// let p = fishers_exact(&[1,9,11,3], TestTails::Two);
///
/// assert_eq!(0.001346 * 1e6, (p * 1e6).round())
/// ```
pub fn fishers_exact(table: &[i32;4], test_tails: TestTails) -> f64 {
    let mut matrix = table.clone();
    let num_tails: i32 = match test_tails {
        TestTails::One => 1,
        TestTails::Two => 2,
    };
    unsafe { r_fishers_exact(matrix.as_mut_ptr(), num_tails) }
}

#[cfg(test)]
mod tests {
    use super::fishers_exact;
    use super::TestTails;

    #[test]
    fn it_works() {

        // The examples from:
        // https://en.wikipedia.org/wiki/Fisher%27s_exact_test

        let cases = [
            ([1, 9, 11, 3], TestTails::Two, 0.001346 * 1e6),
            ([1, 9, 11, 3], TestTails::One, 0.002759 * 1e6),
            ([1, 11, 9, 3], TestTails::Two, 0.001346 * 1e6),
            ([1, 11, 9, 3], TestTails::One, 0.002759 * 1e6),
            ([0, 12, 10, 2], TestTails::Two, 0.000034 * 1e6),
        ];

        for &(table, tails, expected) in cases.iter() {
            let p: f64 = fishers_exact(&table, tails);
            assert_eq!(expected, (p * 1e6).round());
        }
    }
}
