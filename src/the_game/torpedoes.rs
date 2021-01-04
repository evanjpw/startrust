//!

use std::io::BufRead;

use termcolor::WriteColor;

use crate::interaction::beep;
use crate::interaction::getcourse;
use crate::the_game::commands::Command;
use crate::the_game::path::do_path;
use crate::the_game::GameState;
use crate::{StResult, TheGame};

pub fn do_torpedoes<R: BufRead, W: WriteColor>(
    the_game: &mut TheGame,
    sin: &mut R,
    sout: &mut W,
    command: &mut Command,
    gamecomp: &mut GameState,
) -> StResult<()> {
    if the_game.damage.is_damaged(4, false) {
        // Torpedoes damaged
        write!(sout, "SPACE CRUD BLOCKING TUBES.  ")?;
        sout.flush()?;
        let i = 4;
        the_game.damage.show_est_repair_time(sout, i)?;
        beep();
        return Ok(());
    }
    let n: f64 = 15.0;
    if the_game.photo_torpedoes < 1 {
        writeln!(sout, "NO TORPEDOES LEFT!")?;
        return Ok(());
    }
    the_game.course = 10.0;
    while the_game.course >= 9.0 {
        write!(sout, "TORPEDO ")?;
        sout.flush()?;

        the_game.course = getcourse(sin, sout)?;
    }
    if the_game.course < 1.0 {
        // Abort firing of torpedo
        return Ok(());
    }
    the_game.photo_torpedoes -= 1;
    write!(sout, "TRACK: ")?;
    sout.flush()?;
    do_path(the_game, sout, *command, n)?;
    *command = the_game.saved_command;
    // let i = n;
    if the_game.energy <= 0.0 {
        /* Ran out of energy */
        *gamecomp = (-1).into();
    }
    the_game.check_for_hits(sout)?;
    if the_game.energy <= 0.0 {
        /* Ran out of energy */
        *gamecomp = (-1).into();
    }
    if the_game.total_klingons < 1 {
        /* All Klingons destroyed! */
        *gamecomp = 1.into();
    }
    if !gamecomp.is_done() {
        the_game.check_condition();
    }
    Ok(())
}
