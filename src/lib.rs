#![feature(libc)]
extern crate libc;
use libc::{c_double, c_int};

extern "C" {
    fn r_fishers_exact(matrix_2x2: *mut c_int, tails: i32) -> c_double;
}

#[derive(Clone, Copy)]
pub enum TestTails {
    One,
    Two
}

pub fn fishers_exact(matrix_2x2: &[i32;4], test_tails: TestTails) -> f64 {
    let mut matrix = matrix_2x2.clone();
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
