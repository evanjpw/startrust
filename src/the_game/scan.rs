//!

use termcolor::{ColorSpec, WriteColor};

use crate::the_game::damage::Component;
use crate::the_game::quadrant::Quadrant;
use crate::{StResult, TheGame};

/// Set up string for lr scan or galactic records
fn qstr<W: WriteColor>(
    the_game: &TheGame,
    sout: &mut W,
    i: i32,
    j: i32,
    is_current: bool,
) -> StResult<()> {
    let quadrant = Quadrant::new(i, j);
    // The printf format string was "%3.3i", which has a width of 3 digits and has leading 0s.
    // I _think_.
    let value = the_game.quadrant_map[quadrant];
    let emphasize = is_current;
    value.draw(sout, emphasize)?;
    Ok(())
} /* End qstr */

/// Do long-range scan
pub fn l_range_scan<W: WriteColor>(the_game: &mut TheGame, sout: &mut W) -> StResult<()> {
    let i = Component::LongRangeSensors; // Component #2
    if the_game.damage.is_damaged(i.into(), false) {
        // Long-range scan inoperative
        the_game.damage.show_damage(sout, i)?;
        return Ok(());
    }
    let q1: i32 = the_game.q1 as i32;
    let q2: i32 = the_game.q2 as i32;
    writeln!(sout, "{} FOR QUADRANT {} - {}", i.as_ref(), q1 + 1, q2 + 1)?;
    for i in (q1 - 1)..=(q1 + 1) {
        for j in (q2 - 1)..=(q2 + 1) {
            write!(sout, "   ")?;
            sout.flush()?;
            if (i < 0) || (i > 7) || (j < 0) || (j > 7) {
                sout.set_color(ColorSpec::new().set_dimmed(true))?;
                write!(sout, "***")?;
                sout.flush()?;
                sout.reset()?;
            } else {
                let quadrant = Quadrant::new(i as i32, j as i32);
                the_game.quadrant_map[quadrant].show();
                qstr(
                    the_game,
                    sout,
                    i as i32,
                    j as i32,
                    the_game.is_current_quadrant(i, j),
                )?;
            }
        }
        writeln!(sout)?;
    }
    Ok(())
} /* End lrscan */

/// Do galactic records
pub fn galactic_records<W: WriteColor>(the_game: &TheGame, sout: &mut W) -> StResult<()> {
    let i = Component::GalacticRecords; // Component #5
    if the_game.damage.is_damaged(i.into(), false) {
        // Galactic records inoperative
        the_game.damage.show_damage(sout, i)?;
        return Ok(());
    }
    writeln!(
        sout,
        "CUMULATIVE GALACTIC MAP FOR STARDATE {}",
        the_game.current_stardate
    )?;
    for i in 0..8 {
        for j in 0..8 {
            write!(sout, "  ")?;
            sout.flush()?;
            qstr(
                the_game,
                sout,
                i as i32,
                j as i32,
                the_game.is_current_quadrant(i, j),
            )?;
        }
        writeln!(sout)?;
    }
    Ok(())
} /* End galrecs */

/// Do short-range scan
pub fn s_range_scan<W: WriteColor>(the_game: &mut TheGame, sout: &mut W, a: i32) -> StResult<()> {
    the_game.check_condition(); //?
    if a == 0
    /* Initial entry into quadrant */
    {
        the_game.check_for_hits(sout)?;
        if the_game.energy <= 0.0 {
            /* Ran out of energy! */
            return Ok(());
        }
    }
    let i = Component::ShortRangeSensors; // Component #1
    if the_game.damage.is_damaged(i.into(), false) {
        // Short-range scan inoperative
        the_game.damage.show_damage(sout, i)?;
        return Ok(());
    }
    for i in 0..8 {
        for j in 0..8 {
            write!(sout, "{} ", the_game.sector_map.sector_char_at_coords(i, j))?;
            sout.flush()?;
        }
        write!(sout, "  ")?;
        sout.flush()?;
        match i {
            0 => {
                writeln!(
                    sout,
                    "YEARS = {}",
                    the_game.game_defs.ending_stardate - the_game.current_stardate
                )?;
            }
            1 => {
                writeln!(sout, "STARDATE = {}", the_game.current_stardate)?;
            }
            2 => {
                write!(sout, "CONDITION: ")?;
                sout.set_color(&the_game.current_condition.get_color_spec())?;
                writeln!(sout, "{}", the_game.current_condition.as_ref())?;
                sout.reset()?;
            }
            3 => {
                writeln!(sout, "QUADRANT = {} - {}", the_game.q1 + 1, the_game.q2 + 1)?;
            }
            4 => {
                writeln!(sout, "SECTOR = {} - {}", the_game.s1 + 1, the_game.s2 + 1)?;
            }
            5 => {
                writeln!(sout, "ENERGY = {:03}", the_game.energy)?; // printf format string was "%.3f"
            }
            6 => {
                writeln!(
                    sout,
                    "{} = {}",
                    Component::PhotonTorpedoes.as_ref(),
                    the_game.photo_torpedoes
                )?;
            }
            7 => {
                writeln!(sout, "KLINGONS LEFT = {}", the_game.total_klingons)?;
            }
            _ => {}
        }
    }
    Ok(())
} /* End srscan */
