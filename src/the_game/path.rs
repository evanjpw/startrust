//!

use std::f64::consts::FRAC_PI_4;

use termcolor::WriteColor;

use crate::the_game::commands::Command;
use crate::the_game::quadrant::{Quadrant, QuadrantContents};
use crate::the_game::{Sector, SectorContents};
use crate::util::{gt, lt};
use crate::{StResult, StarTrustError, TheGame};

/// Do the path for warp orself torpedo
pub fn do_path<W: WriteColor>(
    the_game: &mut TheGame,
    sout: &mut W,
    command: Command,
    n: f64,
) -> StResult<()> {
    let mut y1 = the_game.s1 as f64 + 0.5;
    let mut x1 = the_game.s2 as f64 + 0.5;
    let mut y3 = (the_game.course - 1.0) as f64 * FRAC_PI_4; // `FRAC_PI_4` _was_ `0.785398`
    let x3 = y3.cos();
    y3 = -(y3.sin());
    let mut in_quadrant = true;
    let mut short_move = command == Command::WarpEngines; // Command #1
    let mut y7 = 0;
    let mut x7 = 0;
    let mut y2 = the_game.game_defs.y2;
    let mut x2 = the_game.game_defs.x2;
    for _ in 0..(n as usize) {
        y1 += y3;
        x1 += x3;
        y2 = y1.floor();
        x2 = x1.floor();
        y7 = y2 as i32;
        x7 = x2 as i32;
        if (x7 < 0) || (x7 > 7) || (y7 < 0) || (y7 > 7) {
            in_quadrant = false;
            short_move = false;
            break;
        }
        if command == Command::PhotonTorpedos
        // Command #5
        {
            // Show torpedo track
            write!(sout, "{} - {}  ", y7 + 1, x7 + 1)?;
            sout.flush()?;
        }
        if the_game
            .sector_map
            .sector_contents_at_coords(y7 as i32, x7 as i32)
            != SectorContents::Empty
        // Content type 1
        {
            // Object blocking move or hit by torpedo
            short_move = false;
            break;
        }
    }

    if in_quadrant {
        // Still in quadrant -- short move, block, or torpedo hit
        the_game.new_quadrant = false;
        writeln!(sout)?;
        if !short_move {
            if command == Command::WarpEngines
            // Command #1
            {
                write!(sout, "BLOCKED BY ")?;
                sout.flush()?;
            }
            match the_game
                .sector_map
                .sector_contents_at_coords(y7 as i32, x7 as i32)
            {
                SectorContents::Klingon => {
                    // case 3 :
                    // Klingon
                    write!(sout, "KLINGON")?;
                    sout.flush()?;
                    if command == Command::PhotonTorpedos
                    // Command #5
                    {
                        // Torpedo
                        for i in 0..8 {
                            if (y7 == the_game.k1[i] as i32) && (x7 == the_game.k2[i] as i32) {
                                the_game.k3[i] = 0.0;
                            }
                        }
                        the_game.quadrant_klingons -= 1;
                        the_game.total_klingons -= 1;
                    }
                }
                SectorContents::Starbase => {
                    // case 4 :
                    // Starbase
                    write!(sout, "STARBASE")?;
                    sout.flush()?;
                    if command == Command::PhotonTorpedos
                    // Command #5
                    {
                        // Torpedo
                        the_game.quadrant_starbases = 2;
                    }
                }
                SectorContents::Star => {
                    // case 5 :
                    // Star
                    write!(sout, "STAR")?;
                    sout.flush()?;
                    if command == Command::PhotonTorpedos
                    // Command #5
                    {
                        // Torpedo
                        the_game.quadrant_stars -= 1;
                    }
                }
                _ => {
                    return Err(StarTrustError::GameStateError(
                        "Ship blocked by unknown object".to_string(),
                    ))
                }
            }
            if command == Command::WarpEngines
            // Command #1
            {
                // Enterprise move
                writeln!(sout, " AT SECTOR {} - {}", y7 + 1, x7 + 1)?;
                y2 = (y1 - y3).floor();
                x2 = (x1 - x3).floor();
                y7 = y2 as i32;
                x7 = x2 as i32;
            }
        }
        if command == Command::WarpEngines
        // Command #1
        {
            the_game.s1 = y2 as i32;
            the_game.s2 = x2 as i32;
            let the_sector = the_game.current_sector();
            the_game.sector_map[the_sector] = 2;
            // Flag to show we stayed within quadrant
            the_game.saved_command = 2.into();
        } else if command == Command::PhotonTorpedos
        // Command #5
        {
            // Torpedo
            write!(sout, " DESTROYED!")?;
            sout.flush()?;
            if the_game.quadrant_starbases == 2 {
                the_game.quadrant_starbases = 0;
                write!(sout, " . . . GOOD WORK!")?;
                sout.flush()?;
            }
            writeln!(sout)?;
            let old_sector = Sector::new(y7 as i32, x7 as i32);
            the_game.sector_map[old_sector] = SectorContents::Empty.into(); // Clear old sector (set it to 1)
            let current_quadrant = Quadrant::new(the_game.q1, the_game.q2);
            the_game.quadrant_map[current_quadrant] = QuadrantContents::new(
                the_game.quadrant_klingons,
                the_game.quadrant_starbases,
                the_game.quadrant_stars,
                false,
            );
        }
    } else {
        // Out of quadrant -- move to new quadrant or torpedo miss
        if command == Command::WarpEngines
        // Command #1
        {
            // Move
            the_game.new_quadrant = true;
            the_game.q1 = (the_game.q1 as f64
                + the_game.warp * y3
                + (the_game.s1 as f64 + 0.5) / 8.0)
                .floor() as i32;
            the_game.q2 = (the_game.q2 as f64
                + the_game.warp * x3
                + (the_game.s2 as f64 + 0.5) / 8.0)
                .floor() as i32;
            the_game.q1 = (the_game.q1 as i32 - lt(the_game.q1 as f64, 0.0)
                + gt(the_game.q1 as f64, 7.0)) as i32;
            the_game.q2 = (the_game.q2 as i32 - lt(the_game.q2 as f64, 0.0)
                + gt(the_game.q2 as f64, 7.0)) as i32;
            the_game.normalize_current_quadrant();
        } else if command == Command::PhotonTorpedos
        // Command #5
        {
            // Torpedo
            writeln!(sout, "MISSED!")?;
        }
    }
    Ok(())
} /* End dopath */
