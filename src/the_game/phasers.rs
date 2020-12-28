use std::io::BufRead;

use log::debug;
use termcolor::WriteColor;

use crate::interaction::{getinp, InputMode, InputValue};
use crate::the_game::damage::Component;
use crate::the_game::{GameState, Sector};
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
            if x <= the_game.energy {
                break;
            }
            writeln!(sout, "ONLY GOT {:03}", the_game.energy)?; // The printf format was "%.3f"
        }
        the_game.energy -= x;
        let y3 = the_game.quadrant_klingons as f64;
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
                    the_game.quadrant_klingons -= 1;
                    the_game.total_klingons -= 1;
                    let sector = Sector::new(the_game.k1[i], the_game.k2[i]);
                    the_game.sector_map[sector] = 1;
                    let quadrant = the_game.current_quadrant();
                    the_game.quadrant_map[quadrant].decrement_klingons();
                }
            }
        }

        if x > 0.0 {
            if the_game.energy <= 0.0 {
                /* Ran out of energy */
                gamecomp = (-1).into();
            }
            the_game.check_for_hits(sout)?;
            if the_game.energy <= 0.0 {
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

/// Determine damage hit amount (distance-dependent)
pub fn fnd(k1_i: i32, k2_i: i32, s1: i32, s2: i32) -> f64 {
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
