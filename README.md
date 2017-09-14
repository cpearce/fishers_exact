# Fisher's exact test from the [R Project](https://www.r-project.org/) in a Rust crate.

Passed to the fishers_exact function to express whether to perform a
one or two tailed statistical test.

Computes the [Fisher's exact test](https://en.wikipedia.org/wiki/Fisher%27s_exact_test)
to determine if there are nonrandom associations between two
categorical variables, in a two by two contingency table.

The test is computed using the Fisher's exact test code copied from
the [R statistical computing package](https://www.r-project.org/).

Given a one dimensional representation of a two by two matrix,
[a,b,c,d], the test computes:

p = ((a + b)!(c + d)!(a + c)!(b + d)!) / (a! b! c! d! (a+b+c+d)!)

Either a one tailed or two tailed test can be computed.

```
use fishers_exact::{fishers_exact,TestTails};

let p = fishers_exact(&[1,9,11,3], TestTails::Two);

assert_eq!(0.001346 * 1e6, (p * 1e6).round())
```