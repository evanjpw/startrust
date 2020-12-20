//! This is an adaptation of an old text-based Star Trek game!
//! This C program is based on a BASIC program adapted by
//! L.E. Cochran 2/29/1976.  In keeping with the original
//! BASIC paradigm, almost all variables and constants are
//! global.
//!
//! To show how much can be done with a relatively small amount
//! of code, the BASIC program was about 2-1/2 pages (150 lines),
//! although almost every line included multiple statements.
//!
//! Bob Sorem -- 28 November 2000

extern crate startrust;

use startrust::{
    clrscr, showinst, title, yesno, StResult, StarTrustError, TheGame, TheGameDefs,
    TheGameDefsBuilder,
};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufRead, Write};

fn get_game_config() -> StResult<TheGameDefs> {
    let the_game_defs = TheGameDefsBuilder::default()
        .build()
        .map_err(|e| StarTrustError::GeneralError(e))?;
    Ok(the_game_defs)
}

fn main() -> Result<(), StarTrustError> {
    let sin = stdin();
    let mut sout = stdout();
    title(&mut sout)?;
    showinst(&sin.lock(), &mut sout)?;

    let the_game_config = get_game_config()?;

    let mut the_game = TheGame::new(&the_game_config);

    loop {
        clrscr(&mut sout)?;
        title(&mut sout)?;

        let _game = the_game.play()?;

        let _ = write!(sout, "\nTRY AGAIN? ").map_err(|e| {
            let e: StarTrustError = e.into();
            e
        })?;
        let ans = yesno(&sin.lock(), &mut sout)?;
        if ans != 'Y' {
            return Ok(());
        }
    }
} /* End main */
