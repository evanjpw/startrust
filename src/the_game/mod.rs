//! # startrust::the_game

#[allow(unused_imports)]
use std::io::{BufRead, Write};
use std::str::FromStr;

use log::debug;
use num_enum::{FromPrimitive, IntoPrimitive};
use strum_macros::{AsRefStr, EnumString};
use termcolor::{Color, ColorSpec, WriteColor};
use unwrap_infallible::UnwrapInfallible;

use crate::error::StarTrustError::GameStateError;
use crate::interaction::{getinp, InputValue};
use crate::the_game::commands::Command;
pub use crate::the_game::config::{TheGameDefs, TheGameDefsBuilder};
use crate::the_game::damage::Damage;
use crate::the_game::phasers::{fnd, phasers};
use crate::the_game::quadrant::{setup_quadrant, Quadrant, QuadrantContents, QuadrantMap};
use crate::the_game::scan::{galactic_records, l_range_scan, s_range_scan};
pub use crate::the_game::sector::{find_slot, Sector, SectorContents, SectorMap};
use crate::the_game::stardate::StarDate;
use crate::the_game::torpedoes::do_torpedoes;
use crate::the_game::warp::do_warp;
use crate::util::{get_random_x_y, gt, lt, rand_init, rnd};
use crate::{yesno, StResult, StarTrustError};

mod commands;
mod config;
mod damage;
mod path;
mod phasers;
mod quadrant;
mod scan;
mod sector;
mod stardate;
mod torpedoes;
mod warp;

#[derive(Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(i32)]
pub enum GameState {
    #[num_enum(default)]
    InProgress = 0,
    Won = 1,
    Lost = -1,
    Quit = -99,
}

impl GameState {
    fn is_done(&self) -> bool {
        !matches!(self, GameState::InProgress)
    }

    fn update(&mut self, new_game_state: GameState) {
        if *self == GameState::InProgress {
            *self = new_game_state
        }
    }
}

#[derive(AsRefStr, Debug, PartialEq, EnumString)]
enum Condition {
    #[strum(serialize = "RED")]
    Red,
    #[strum(serialize = "YELLOW")]
    Yellow,
    #[strum(serialize = "GREEN")]
    Green,
    #[strum(serialize = "DOCKED")]
    Docked,
    #[strum(serialize = "")] // UNDEFINED
    Undefined,
}

impl Condition {
    fn get_color_spec(&self) -> ColorSpec {
        match self {
            Condition::Red => {
                let mut c = ColorSpec::new();
                c.set_fg(Some(Color::Red));
                c
            }
            Condition::Yellow => {
                let mut c = ColorSpec::new();
                c.set_fg(Some(Color::Yellow));
                c
            }
            Condition::Green => {
                let mut c = ColorSpec::new();
                c.set_fg(Some(Color::Green));
                c
            }
            Condition::Docked => {
                let mut c = ColorSpec::new();
                c.set_fg(Some(Color::Cyan));
                c
            }
            Condition::Undefined => ColorSpec::new(),
        }
    }
}

// The interpretation of (s1, s2) and (q1, q2) is unclear. Most of the code, the instructions, and
// most of the game play suggest that they are (y, x), which is weird, but there are also places in
// the code where they are used as if they are (x, y). It appears that (y, x) was the way it was
// initially coded, but then someone modifying or translating the code, or even the author, became
// confused and thought that it was (x, y). This rewrite preserves the confusion, for the time
// being, because it is trying to recreate the 1978 game accurately. This is also why the ambiguous
// names were preserved.

pub struct TheGame {
    /// Current Energy
    energy: f64,
    /// Current Photon Torpedoes
    photo_torpedoes: i32,
    /// Current StarDate
    current_stardate: StarDate,
    /// Total remaining Klingons
    total_klingons: i32,
    /// The Enterprise's x position within the Quadrant
    s1: i32,
    /// The Enterprise's y position within the Quadrant
    s2: i32,
    /// The x position of the current Quadrant
    q1: i32,
    /// The y position of the current Quadrant
    q2: i32,
    /// The Damage Array
    damage: Damage,
    /// Klingons Destroyed
    klingons_destroyed: i32,
    /// The number of Starbases
    total_starbases: i32,
    /// New Quadrant
    new_quadrant: bool,
    /// The Sector Map
    pub(crate) sector_map: SectorMap,
    /// The Quadrant Map
    pub(crate) quadrant_map: QuadrantMap,
    /// Configured game starting values
    pub(crate) game_defs: TheGameDefs,
    /// The current condition of the Enterprise
    current_condition: Condition,
    /// The current command (used as a flag to initiate certain behavior)
    saved_command: Command,
    /// The ending stardate for the game
    ending_stardate: StarDate,
    /// Starbases in the current quadrant
    quadrant_starbases: i32,
    /// Klingons in the current quadrant
    quadrant_klingons: i32,
    /// Course
    course: f64,
    /// Warp
    warp: f64,
    k1: Vec<i32>,
    k2: Vec<i32>,
    k3: Vec<f64>,
    s: i32,
}

impl TheGame {
    pub fn new(the_game_defs: &TheGameDefs) -> Self {
        // s1 & s2 and q1 & q2 are not set initially, we will use (0, 0) the game
        // initialization will randomize them later
        let b9 = 0;
        let c = 100_f64;
        let w = 10_f64;
        Self {
            energy: the_game_defs.initial_energy,
            photo_torpedoes: the_game_defs.initial_photon_torpedoes,
            current_stardate: the_game_defs.beginning_stardate,
            total_klingons: the_game_defs.initial_total_klingons,
            sector_map: SectorMap::new(),
            quadrant_map: QuadrantMap::new(),
            s1: 0,
            s2: 0,
            q1: 0,
            q2: 0,
            damage: Damage::new(),
            klingons_destroyed: 0,
            k1: vec![0i32; 8],
            k2: vec![0i32; 8],
            k3: vec![0.0; 8],
            game_defs: *the_game_defs,
            total_starbases: b9,
            new_quadrant: false,
            quadrant_klingons: 0,
            course: c,
            warp: w,
            quadrant_starbases: 0,
            current_condition: Condition::Undefined,
            saved_command: Command::Undefined, // the global version of `a`
            s: 0,
            ending_stardate: the_game_defs.ending_stardate,
        }
    }

    fn normalize_current_quadrant(&mut self) {
        if self.q1 > 7 {
            self.q1 = 7;
        } else if self.q1 < 0 {
            self.q1 = 0;
        };
        if self.q2 > 7 {
            self.q2 = 7;
        } else if self.q2 < 0 {
            self.q2 = 0;
        };
    }

    /// Initialize
    pub fn init<W: WriteColor>(&mut self, sout: &mut W) -> StResult<()> {
        rand_init();
        self.damage.fix_damage();
        let (x, y) = get_random_x_y();
        self.set_current_quadrant_from_coords(x, y);

        let x = 8;
        let y = 1;
        let mut total_starbases = self.total_starbases;

        let the_game_defs = self.game_defs;
        let mut ending_stardate = the_game_defs.ending_stardate;
        let beginning_stardate = self.beginning_stardate();
        let mut total_klingons = self.total_klingons as i32;
        let x1 = self.game_defs.x1;
        let x2 = self.game_defs.x2;
        let y1 = self.game_defs.y1;
        let y2 = self.game_defs.y2;
        let mut klingons = self.quadrant_klingons;
        let mut starbases;

        for i in 0..8 {
            for j in 0..8 {
                klingons = 0;
                let mut n = rnd();
                if n < x1 {
                    n *= 64.0;
                    klingons = lt(n, y1) as i32 - (y as i32);
                    klingons = -(klingons
                        + lt(n, x2) as i32
                        + lt(n, y2) as i32
                        + lt(n, 0.08) as i32
                        + lt(n, 0.03) as i32
                        + lt(n, 0.01) as i32);
                    total_klingons += klingons as i32;
                }

                starbases = -gt(rnd(), self.game_defs.starbase_frequency);
                total_starbases += starbases;

                let stars = (rnd() * (x as f64) + (y as f64)).floor() as i32;

                let quadrant = Quadrant::new(i, j);
                self.quadrant_map[quadrant] =
                    QuadrantContents::new(klingons, starbases, stars, true);
            }
        }

        // Ensure that there are not more Klignons than years
        if total_klingons > (ending_stardate - beginning_stardate) as i32 {
            ending_stardate = beginning_stardate + total_klingons as i32;
        }

        // Ensure that there is at least one starbase
        if total_starbases <= 0 {
            let (starbase_x, starbase_y) = get_random_x_y();
            let quadrant = Quadrant::new(starbase_x, starbase_y);
            let mut quadrant_value = self.quadrant_map[quadrant];
            debug!(
                "About to add one to quadrant {} with value {:?}",
                quadrant, quadrant_value
            );

            quadrant_value.starbases += 1;
            debug!(
                "About to store value {:?} in quadrant {}",
                quadrant_value, quadrant
            );
            self.quadrant_map[quadrant] = quadrant_value;
            total_starbases = 1;
        }

        let years = ending_stardate - beginning_stardate;
        writeln!(
            sout,
            "OBJECTIVE: DESTROY {} KLINGON BATTLE CRUISERS IN {} YEARS.",
            total_klingons, years
        )?;
        writeln!(sout, " THE NUMBER OF STARBASES IS {}.\n", total_starbases)?;

        self.quadrant_klingons = klingons;
        self.total_klingons = total_klingons;
        self.klingons_destroyed = total_klingons;
        self.total_starbases = total_starbases;
        self.ending_stardate = ending_stardate;

        Ok(())
    } /* End init */

    #[allow(dead_code)]
    fn years(&self) -> i32 {
        self.ending_stardate - self.beginning_stardate()
    }

    pub fn increment_year(&mut self) {
        self.current_stardate += 1i32;
    }

    fn current_sector(&self) -> Sector {
        Sector::new(self.s1, self.s2)
    }

    fn set_current_sector(&mut self, sector: Sector) {
        self.set_current_sector_from_coords(sector.x(), sector.y())
    }

    fn set_current_sector_from_coords(&mut self, x: i32, y: i32) {
        self.s1 = x;
        self.s2 = y;
    }

    fn current_quadrant(&self) -> Quadrant {
        Quadrant::new(self.q1, self.q2)
    }

    #[allow(dead_code)]
    fn set_current_quadrant(&mut self, quadrant: Quadrant) {
        self.set_current_quadrant_from_coords(quadrant.x(), quadrant.y());
    }

    fn set_current_quadrant_from_coords(&mut self, x: i32, y: i32) {
        self.q1 = x;
        self.q2 = y;
    }

    fn is_current_quadrant(&self, x: i32, y: i32) -> bool {
        x == self.q1 && y == self.q2
    }

    /// Display current star date
    pub fn show_stardate<W: WriteColor>(&self, sout: &mut W) -> StResult<()> {
        write!(sout, "\nIT IS STARDATE {}.\n", self.current_stardate)?;
        sout.flush().map_err(|e| {
            let e: StarTrustError = e.into();
            e
        })
    } /* End showstardate */

    /// Check condition
    fn check_condition(&mut self) {
        let s1 = self.s1 as i32;
        let s2 = self.s2 as i32;
        let e0 = self.game_defs.initial_energy;
        let p0 = self.game_defs.initial_photon_torpedoes;
        for i in (s1 - 1)..=(s1 + 1) {
            for j in (s2 - 1)..=(s2 + 1) {
                if (i >= 0) && (i <= 7) && (j >= 0) && (j <= 7) {
                    let sector = Sector::new(i as i32, j as i32);
                    if self.sector_map[sector] == SectorContents::Starbase.into() {
                        // Docked at starbase
                        self.current_condition = Condition::Docked;
                        self.energy = e0;
                        self.photo_torpedoes = p0;
                        self.damage.fix_damage();
                        return;
                    }
                }
            }
        }
        if self.quadrant_klingons > 0 {
            // Klingons present!
            self.current_condition = Condition::Red;
        } else if self.energy < (0.1 * e0) {
            // Low energy
            self.current_condition = Condition::Yellow;
        } else {
            // A-OK!
            self.current_condition = Condition::Green;
        }
    } /* End checkcond */

    /// Show hit on Enterprise or Klingon
    fn show_hit<W: WriteColor>(
        &self,
        sout: &mut W,
        i: usize,
        es: &str,
        n: f64,
        h: f64,
    ) -> StResult<()> {
        writeln!(
            sout,
            "{:.3} UNIT HIT ON {} SECTOR {} - {}  ({:.3} LEFT)",
            h,
            es,
            self.k1[i] + 1,
            self.k2[i] + 1,
            n
        )
        .map_err(|e| e.into())
    } /* End showhit */

    fn is_docked(&self) -> bool {
        // This is an amazingly stupid way to do this, but it's how they do it
        self.current_condition == Condition::Docked
    }

    /// Check for hits from Klingons
    fn check_for_hits<W: WriteColor>(&mut self, sout: &mut W) -> StResult<()> {
        if self.quadrant_klingons < 1 {
            /* No Klingons here! */
            return Ok(());
        }
        if self.is_docked() {
            writeln!(sout, "STARBASE PROTECTS ENTERPRISE.")?;
            return Ok(());
        }
        for i in 0..8 {
            if self.k3[i] > 0.0 {
                let mut h = self.k3[i] * 0.4 * rnd();
                self.k3[i] -= h;
                h /= fnd(self.k1[i], self.k2[i], self.s1, self.s2).powf(0.4);
                self.energy -= h;
                let n: f64 = self.energy;
                self.show_hit(sout, i, "ENTERPRISE FROM", n, h)?;
            }
        }
        Ok(())
    } /* End checkforhits */

    pub fn s9(&self) -> f64 {
        self.game_defs.s9
    }

    pub fn beginning_stardate(&self) -> StarDate {
        self.game_defs.beginning_stardate
    }

    pub fn play<R: BufRead, W: WriteColor>(&mut self, sin: &mut R, sout: &mut W) -> StResult<()> {
        let mut gamecomp = GameState::InProgress;
        let mut moved: bool = false;
        let mut command = self.saved_command;

        debug!(
            "Init gamecomp={:?}, moved={}, a={:?}",
            gamecomp, moved, command
        );
        self.init(sout)?;
        self.new_quadrant = true;
        debug!(
            "Done initing gamecomp={:?}, moved={}, a={:?}, newquad={}",
            gamecomp, moved, command, self.new_quadrant
        );

        while !gamecomp.is_done() {
            if self.new_quadrant {
                setup_quadrant(self);
                command = self.saved_command;
            }
            self.new_quadrant = false;
            moved = false;
            s_range_scan(self, sout, command.into())?;
            if self.energy <= 0.0 {
                /* Ran out of energy */
                gamecomp = GameState::Lost;
            } else {
                loop
                /* Command loop (-99 or ESC to quit) */
                {
                    write!(sout, "COMMAND? ")?;
                    sout.flush()?;
                    let ebuff = getinp(sin, sout, 7, 2.into())?;
                    writeln!(sout)?;
                    match ebuff {
                        InputValue::Blank => command = Command::Undefined,
                        InputValue::Esc => command = (-99).into(),
                        InputValue::InputString(cmdbuff) => {
                            command = Command::from_str(cmdbuff.as_str()).unwrap_infallible();
                        }
                    }
                    match command {
                        Command::WarpEngines => {
                            //case 1 :
                            // Warp engines
                            do_warp(self, sin, sout, &mut command, &mut gamecomp, &mut moved)?;
                        }
                        Command::ShortRangeScan => {
                            //case 2 :
                            // Short-range scan
                            s_range_scan(self, sout, command.into())?;
                        }
                        Command::LongRangeScan => {
                            //case 3 :
                            /* Long-range scan */
                            l_range_scan(self, sout)?;
                        }
                        Command::Phasers => {
                            //case 4 :
                            /* Phasers */
                            let x = phasers(self, sin, sout)?;
                            gamecomp.update(x);
                        }
                        Command::PhotonTorpedos => {
                            //case 5 :
                            // Photon torpedoes
                            do_torpedoes(self, sin, sout, &mut command, &mut gamecomp)?;
                        }
                        Command::GalacticRecords => {
                            //case 6 :
                            /* Galactic records */
                            galactic_records(self, sout)?;
                        }
                        Command::Quit => {
                            write!(sout, "\nARE YOU SURE YOU WANT TO QUIT? ")?;
                            sout.flush()?;
                            let ans = yesno(sin)?;
                            if ans == 'Y' {
                                gamecomp = (-99).into();
                                break; /* Break out of command loop */
                            } else {
                                continue;
                            } /* Back to top of command loop */
                        }
                        Command::Undefined => {
                            debug!("undefined command in command loop.");
                            for i in 1..7 {
                                let command: Command = i.into();
                                writeln!(sout, "  {} = {}", i, command)?;
                            }
                            writeln!(sout, "  -99 OR ESC TO QUIT\n")?;
                            // Back to top of command loop
                            continue;
                        }
                    }

                    if gamecomp.is_done() {
                        break;
                    }
                    if moved {
                        // Enterprise moved
                        break;
                    }
                } /* End command loop */
            }
        } /* Game is over! */

        self.show_stardate(sout)?;
        match gamecomp {
            GameState::Won => {
                let t = self.current_stardate;
                let t0 = self.beginning_stardate();
                let drate: f64 = (t - t0) as f64;
                let rating: i32 = ((self.klingons_destroyed as f64 / drate) * 1000.0) as i32;
                writeln!(sout, "THE FEDERATION HAS BEEN SAVED!")?;
                writeln!(sout, "YOU ARE PROMOTED TO ADMIRAL.")?;
                writeln!(
                    sout,
                    "{} KLINGONS IN {} YEARS.  RATING = {}\n",
                    self.klingons_destroyed,
                    t - t0,
                    rating,
                )?;
            }
            GameState::Lost => {
                if self.current_stardate > self.game_defs.ending_stardate {
                    writeln!(sout, "YOU RAN OUT OF TIME!")?;
                } else if self.energy <= 0.0 {
                    writeln!(sout, "YOU RAN OUT OF ENERGY!")?;
                } else {
                    return Err(GameStateError(String::from(
                        "GameState::Lost with no discernible reason",
                    )));
                }
                writeln!(sout, "THANKS TO YOUR BUNGLING, THE FEDERATION WILL BE")?;
                writeln!(
                    sout,
                    "CONQUERED BY THE REMAINING {} KLINGON CRUISERS!",
                    self.total_klingons
                )?;
                writeln!(sout, "YOU ARE DEMOTED TO CABIN BOY!")?;
            }
            GameState::Quit => {
                writeln!(sout, "OKAY, QUITTER -- NO KUDOS FOR YOU.")?;
            }
            GameState::InProgress => {
                return Err(StarTrustError::GameStateError(String::from(
                    "`gamecomp` is `InProgress`, but in game complete",
                )))
            }
        }
        Ok(())
    }
}
