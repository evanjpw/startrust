//!

use std::convert::TryInto;
use std::io::BufRead;

use termcolor::WriteColor;

use crate::interaction::beep;
use crate::interaction::{delay, getcourse, getwarp};
use crate::the_game::commands::Command;
use crate::the_game::damage::Component;
use crate::the_game::GameState;
use crate::util::rnd;
use crate::{StResult, TheGame};

const WARP: Component = Component::WarpEngines; // Component #0

pub fn do_warp<R: BufRead, W: WriteColor>(
    the_game: &mut TheGame,
    sin: &mut R,
    sout: &mut W,
    a: &mut Command,
    gamecomp: &mut GameState,
    moved: &mut bool,
) -> StResult<()> {
    let mut w = 0f64;
    let mut c;

    loop {
        loop {
            c = getcourse(sin, sout)?;
            the_game.c = c;
            if c < 9.0 {
                break;
            }
            beep();
        }
        if c >= 1.0 {
            loop {
                w = getwarp(sin, sout)?;
                if (w <= 0.0) || (w > 12.0) {
                    c = 10.0;
                    break;
                }
                if the_game.damage.is_damaged(WARP.into(), false) && (w > 0.2) {
                    write!(sout, "{} DAMAGED; MAX IS 0.2; ", WARP.as_ref())?;
                    sout.flush()?;
                    the_game.damage.show_est_repair_time(sout, WARP.into())?;
                    beep();
                } else {
                    break;
                }
                beep();
            }
        }
        if c < 9.0 {
            break;
        }
    }
    if c < 1.0 {
        // Abort move
        return Ok(());
    }
    the_game.check_for_hits(sout)?;
    if the_game.e <= 0.0 {
        /* Ran out of energy */
        *gamecomp = (-1).into();
        return Ok(());
    }

    if rnd() <= 0.25 {
        let x = (rnd() * 6.0).floor() as usize;
        if rnd() <= 0.5 {
            beep();
            the_game
                .damage
                .add_damage(x, (6.0 - rnd() * 5.0).floor() as i32);
            let i: Component = x.try_into()?;
            writeln!(sout, "**SPACE STORM, {} DAMAGED**", i)?;
            the_game.damage.show_est_repair_time(sout, x.into())?;
            the_game.damage.add_damage(x, 1);
            delay(100);
            beep();
        } else {
            let mut j: i32 = -1;
            for i in x..6 {
                if the_game.damage.is_damaged(i, false) {
                    j = i as i32;
                    break;
                }
            }
            if j < 0 {
                for i in 0..x {
                    if the_game.damage.is_damaged(i, false) {
                        j = i as i32;
                        break;
                    }
                }
            }
            if j >= 0 {
                the_game.damage.set_damage(j as usize, 1);
                writeln!(sout, "**SPOCK USED A NEW REPAIR TECHNIQUE**")?;
            }
        }
    }
    for i in 0..6 {
        if the_game.damage.is_damaged(i, true) {
            if the_game.damage.reduce_and_normalize_damage(i) {
                let component: Component = i.try_into()?;
                writeln!(sout, "{} ARE FIXED!", component.as_ref())?;
                beep();
            }
        }
    }
    let n = (w * 8.0).floor();
    the_game.w = w;
    the_game.e = the_game.e - n - n + 0.5;
    the_game.current_stardate += 1i32;
    let current_sector = the_game.current_sector();
    the_game.sect[current_sector] = 1;
    if the_game.current_stardate > the_game.game_defs.t9 {
        /* Ran out of time! */
        *gamecomp = (-1).into();
        return Ok(());
    }
    the_game.do_path(sout, *a, n)?;
    *a = the_game.saved_command;
    // let i = n;
    if the_game.e <= 0.0 {
        // Ran out of energy
        *gamecomp = (-1).into();
        return Ok(());
    }
    *moved = true;
    Ok(())
}
