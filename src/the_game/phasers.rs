use std::io::BufRead;

use log::debug;
use termcolor::WriteColor;

use crate::interaction::{getinp, InputMode, InputValue};
use crate::the_game::damage::Component;
use crate::the_game::{GameState, Sector};
use crate::util::fnd;
use crate::{StResult, TheGame};

const PHASERS: Component = Component::Phasers; // Component # 3

/// Fire phasers
pub fn phasers<R: BufRead, W: WriteColor>(
    the_game: &mut TheGame,
    sin: &mut R,
    sout: &mut W,
) -> StResult<GameState> {
    let mut gamecomp = GameState::InProgress;

    if the_game.damage.is_damaged(PHASERS.into(), false) {
        // Phasers inoperative
        the_game.damage.show_damage(sout, PHASERS)?;
    } else {
        let mut x;

        loop {
            write!(sout, "PHASERS READY: ENERGY UNITS TO FIRE? ")?;
            sout.flush()?;
            let gb = getinp(sin, sout, 15, InputMode::Mode2)?;
            writeln!(sout)?;
            if let InputValue::InputString(ibuff) = gb {
                x = ibuff.parse()?;
            } else {
                x = 0.0;
                break;
            }
            if x <= the_game.e {
                break;
            }
            writeln!(sout, "ONLY GOT {:03}", the_game.e)?; // The printf format was "%.3f"
        }
        the_game.e -= x;
        let y3 = the_game.k as f64;
        for i in 0..8 {
            if the_game.k3[i] > 0.0 {
                let f = fnd(the_game.k1[i], the_game.k2[i], the_game.s1, the_game.s2);
                debug!("About to fire phasers: x = {}, y3 = {}, f = {}", x, y3, f);
                let h = x / (y3 * f.powf(0.4));
                the_game.k3[i] -= h;
                let n = the_game.k3[i];
                the_game.show_hit(sout, i, "KLINGON AT", n, h)?;
                if the_game.k3[i] <= 0.0 {
                    writeln!(sout, "**KLINGON DESTROYED**")?;
                    the_game.k -= 1;
                    the_game.total_klingons -= 1;
                    let sector = Sector::new(the_game.k1[i], the_game.k2[i]);
                    the_game.sect[sector] = 1;
                    let quadrant = the_game.current_quadrant();
                    the_game.quad[quadrant].decrement_klingons();
                }
            }
        }

        if x > 0.0 {
            if the_game.e <= 0.0 {
                /* Ran out of energy */
                gamecomp = (-1).into();
            }
            the_game.check_for_hits(sout)?;
            if the_game.e <= 0.0 {
                /* Ran out of energy */
                gamecomp = (-1).into();
            }
            if the_game.total_klingons < 1 {
                /* All Klingons destroyed! */
                gamecomp = 1.into();
            }
            if !gamecomp.is_done() {
                the_game.check_condition()
            };
        }
    }
    Ok(gamecomp)
} /* End phasers */
