//! # startrust::util
//!

use crate::the_game::{Sector, SectorContents, SectorMap};
use rand::{thread_rng, Rng};

/// Set a random x and y in interval \[0,7\]
pub fn setrndxy() -> (u8, u8) {
    let x: u8 = thread_rng().gen_range(0..8);
    let y: u8 = thread_rng().gen_range(0..8);
    (x, y)
} /* End setrndxy */

/// Find an unoccupied sector
pub fn findslot(sector_map: &SectorMap) -> Sector {
    loop {
        let (x, y) = setrndxy();
        let sector = Sector::new(x, y);
        if sector_map.sector_contents_at(sector) == SectorContents::Empty {
            return sector;
        }
    }
} /* End findslot */

//doublevoid
/* Initialize pseudo-random number generator */
pub fn randinit() -> f64 {
    /*
    struct time t;
    double r1,r2,r3,r4;

    gettime(&t);
    r1=t.ti_hund;
    r2=t.ti_sec;
    r3=t.ti_min;
    r4=t.ti_hour;
    r2=floor(r2*(100.0/60.0));
    r3=floor(r3*(100.0/60.0));
    r4=floor(r4*(100.0/24.0));
    rn=r1/100.0+r2/10000.0+r3/1000000.0+r4/100000000.0;
    return rn;
    */

    // Does nothing
    0.0f64
} /* End randinit */

//random, , , vvoi**voidvoidwhile (TRUE)sect[][]<2break;(u8, u8)_coordsx, ydoubledoublereturn;
/*return;
doublevoiddoublereturn r-floor(r);doubledoublevoid,,00 return;intdoubledoublereturnreturn;;int
 */

/// Get fractional part of (double) real number
fn frac(r: f64) -> f64 {
    r.fract()
} /* End frac */

/// Generate a new pseudo-random number
pub fn rnd() -> f64 {
    // This used to literally be:
    // ```
    // rn=frac(rn*777.7);
    // return rn;
    // ```
    thread_rng().gen()
} /* End rnd */

/// Determine damage hit amount (distance-dependent)
pub fn fnd(k1_i: u8, k2_i: u8, s1: u8, s2: u8) -> f64 {
    let k1_i = k1_i as f64; // k1[i]
    let k2_i = k2_i as f64; // k2[i]
    let s1 = s1 as f64;
    let s2 = s2 as f64;

    let dx = (k1_i - s1).abs();
    let dy = (k2_i - s2).abs();

    let dx2 = dx.powi(2);
    let dy2 = dy.powi(2);

    (dx2 + dy2).sqrt()
} /* End fnd */

/// See if r1 is less than r2, BASIC style
pub fn lt(r1: f64, r2: f64) -> i32 {
    if (r1 < r2) {
        -1 /* BASIC true = -1 */
    } else {
        0 /* BASIC false = 0 */
    }
} /* End lt */

/// See if r1 is greater than r2, BASIC style
pub fn gt(r1: f64, r2: f64) -> i32 {
    if (r1 > r2) {
        -1 /* BASIC true = -1 */
    } else {
        0 /* BASIC false = 0 */
    }
} /* End gt */
