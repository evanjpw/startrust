/* This is an adaptation of an old text-based Star Trek game!
   This C program is based on a BASIC program adapted by
   L.E. Cochran 2/29/1976.  In keeping with the original
   BASIC paradigm, almost all variables and constants are
   global.

   To show how much can be done with a relatively small amount
   of code, the BASIC program was about 2-1/2 pages (150 lines),
   although almost every line included multiple statements.

   Bob Sorem -- 28 November 2000
*/

extern crate startrust;

use startrust::{clrscr, showinst, title, yesno, StarTrustError};
use std::io::{stdin, stdout};

const QS: &str = ".EKB*";
const DS: &'static [&'static str] = &[
    "WARP ENGINES",
    "SHORT RANGE SENSORS",
    "LONG RANGE SENSORS",
    "PHASERS",
    "PHOTON TORPEDOES",
    "GALACTIC RECORDS",
];

fn main() -> Result<(), StarTrustError> {
    let sin = stdin();
    let mut sout = stdout();
    title(&mut sout);
    clrscr(&mut sout);
    showinst(&sin, &mut sout)?;

    loop {
        clrscr(&mut sout);
        title(&mut sout);

        let game = write!(sout, "\nTRY AGAIN? ").map_err(|e| {
            let e: StarTrustError = e.into();
            e
        });
        let ans = yesno(&sin, &mut sout)?;
        if ans != 'Y' {
            return Ok(());
        }
    }
} /* End main */
