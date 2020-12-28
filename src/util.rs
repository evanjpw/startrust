//! # startrust::util
//!

use rand::{thread_rng, Rng};

/// Set a random x and y in interval \[0,7\]
pub fn get_random_x_y() -> (i32, i32) {
    let x: i32 = thread_rng().gen_range(0..8);
    let y: i32 = thread_rng().gen_range(0..8);
    (x, y)
} /* End setrndxy */

/* Initialize pseudo-random number generator */
pub fn rand_init() -> f64 {
    // Does nothing
    0.0f64
} /* End randinit */

/// Generate a new pseudo-random number
pub fn rnd() -> f64 {
    // This used to literally be:
    // ```
    // rn=frac(rn*777.7);
    // return rn;
    // ```
    thread_rng().gen()
} /* End rnd */

/// See if r1 is less than r2, BASIC style
pub fn lt(r1: f64, r2: f64) -> i32 {
    if r1 < r2 {
        -1 /* BASIC true = -1 */
    } else {
        0 /* BASIC false = 0 */
    }
} /* End lt */

/// See if r1 is greater than r2, BASIC style
pub fn gt(r1: f64, r2: f64) -> i32 {
    if r1 > r2 {
        -1 /* BASIC true = -1 */
    } else {
        0 /* BASIC false = 0 */
    }
} /* End gt */
