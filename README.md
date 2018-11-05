# Fisher's exact test

[![Build Status](https://travis-ci.org/cpearce/fishers_exact.svg?branch=master)](https://travis-ci.org/cpearce/fishers_exact)

Implements a 2×2 Fishers exact test. Use this to test the independence of two
categorical variables when the sample sizes are small.

For an approachable explanation of Fisher's exact test, see
[Fisher's exact test of independence](http://www.biostathandbook.com/fishers.html) by
John H. McDonald in the [Handbook of Biological Statistics](http://www.biostathandbook.com/).

The test is computed using code ported from Øyvind Langsrud's JavaScript
implementation at [http://www.langsrud.com/fisher.htm](http://www.langsrud.com/fisher.htm),
used with permission.

```
use fishers_exact::{fishers_exact,TestTails};

let p = fishers_exact(&[1,9,11,3], TestTails::Two);

assert_eq!(0.001346 * 1e6, (p * 1e6).round())
```