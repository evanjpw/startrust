//!

use std::convert::TryInto;
use std::io::BufRead;

use termcolor::WriteColor;

use crate::interaction::beep;
use crate::interaction::{delay, getcourse, getwarp};
use crate::the_game::commands::Command;
use crate::the_game::damage::Component;
use crate::the_game::path::do_path;
use crate::the_game::GameState;
use crate::util::rnd;
use crate::{StResult, TheGame};

const WARP: Component = Component::WarpEngines; // Component #0

pub fn do_warp<R: BufRead, W: WriteColor>(
    the_game: &mut TheGame,
    sin: &mut R,
    sout: &mut W,
    command: &mut Command,
    gamecomp: &mut GameState,
    moved: &mut bool,
) -> StResult<()> {
    let mut warp = 0f64;
    let mut course;

    loop {
        loop {
            course = getcourse(sin, sout)?;
            the_game.course = course;
            if course < 9.0 {
                break;
            }
            beep();
        }
        if course >= 1.0 {
            loop {
                warp = getwarp(sin, sout)?;
                if (warp <= 0.0) || (warp > 12.0) {
                    course = 10.0;
                    break;
                }
                if the_game.damage.is_damaged(WARP.into(), false) && (warp > 0.2) {
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
        if course < 9.0 {
            break;
        }
    }
    if course < 1.0 {
        // Abort move
        return Ok(());
    }
    the_game.check_for_hits(sout)?;
    if the_game.energy <= 0.0 {
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
            the_game.damage.show_est_repair_time(sout, x)?;
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
        if the_game.damage.is_damaged(i, true) && the_game.damage.reduce_and_normalize_damage(i) {
            let component: Component = i.try_into()?;
            writeln!(sout, "{} ARE FIXED!", component.as_ref())?;
            beep();
        }
    }
    let n = (warp * 8.0).floor();
    the_game.warp = warp;
    the_game.energy = the_game.energy - n - n + 0.5;
    the_game.current_stardate += 1i32;
    let current_sector = the_game.current_sector();
    the_game.sector_map[current_sector] = 1;
    if the_game.current_stardate > the_game.game_defs.ending_stardate {
        /* Ran out of time! */
        *gamecomp = (-1).into();
        return Ok(());
    }
    do_path(the_game, sout, *command, n)?;
    *command = the_game.saved_command;
    // let i = n;
    if the_game.energy <= 0.0 {
        // Ran out of energy
        *gamecomp = (-1).into();
        return Ok(());
    }
    *moved = true;
    Ok(())
}
