//! # startrust::the_game

use std::convert::AsRef;
use std::f64::consts::FRAC_PI_4;
#[allow(unused_imports)]
use std::io::{BufRead, Write};

use log::{debug, error};
use num_enum::{FromPrimitive, IntoPrimitive};
use strum_macros::{AsRefStr, EnumString};
use termcolor::{Color, ColorSpec, WriteColor};

use crate::error::StarTrustError::GameStateError;
use crate::interaction::{beep, delay, getcourse, getinp, getwarp, InputMode, InputValue};
use crate::the_game::commands::Command;
pub use crate::the_game::config::{TheGameDefs, TheGameDefsBuilder};
use crate::the_game::quadrant::{setup_quadrant, Quadrant, QuadrantContents, QuadrantMap};
pub use crate::the_game::sector::{Sector, SectorContents, SectorMap};
use crate::the_game::stardate::StarDate;
use crate::util::{fnd, gt, lt, rand_init, rnd, set_random_x_y};
use crate::{yesno, StResult, StarTrustError};

mod commands;
mod config;
mod quadrant;
mod sector;
mod stardate;

const DS: &'static [&'static str] = &[
    "WARP ENGINES",
    "SHORT RANGE SENSORS",
    "LONG RANGE SENSORS",
    "PHASERS",
    "PHOTON TORPEDOES",
    "GALACTIC RECORDS",
];

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
        match self {
            GameState::InProgress => false,
            _ => true,
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

pub struct TheGame {
    /// Current Energy
    e: f64,
    /// Current Photon Torpedoes
    p: i32,
    /// Current StarDate
    current_stardate: StarDate,
    years: i32,
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
    damage: Vec<i32>,
    k0: i32,
    k1: Vec<i32>,
    k2: Vec<i32>,
    k3: Vec<f64>,
    /// The number of Starbases
    total_starbases: i32,
    /// New Quadrant
    newquad: bool,
    k: i32,
    /// The Sector Map
    pub(crate) sect: SectorMap,
    /// The Quadrant Map
    pub(crate) quad: QuadrantMap,
    pub(crate) game_defs: TheGameDefs,
    c: f64,
    /// Warp
    w: f64,
    b: i32,
    /// The current condition of the Enterprise as a String
    cond: Condition,
    saved_command: Command,
    s: i32,
    ending_stardate: StarDate,
}
impl TheGame {
    pub fn new(the_game_defs: &TheGameDefs) -> Self {
        // s1 & s2 and q1 & q2 are not set initially, we will use (0, 0) the game
        // initialization will randomize them later
        let b9 = 0;
        let c = 100 as f64;
        let w = 10 as f64;
        Self {
            e: the_game_defs.e0,
            p: the_game_defs.p0,
            current_stardate: the_game_defs.t0,
            years: (the_game_defs.t9 - the_game_defs.t0) as i32,
            total_klingons: the_game_defs.k9,
            sect: SectorMap::new(),
            quad: QuadrantMap::new(),
            s1: 0,
            s2: 0,
            q1: 0,
            q2: 0,
            damage: vec![0i32; 6],
            k0: 0,
            k1: vec![0i32; 8],
            k2: vec![0i32; 8],
            k3: vec![0.0; 8],
            game_defs: *the_game_defs,
            total_starbases: b9,
            newquad: false,
            k: 0,
            c,
            w,
            b: 0,
            cond: Condition::Undefined,
            saved_command: Command::Undefined, // the global version of `a`
            s: 0,
            ending_stardate: the_game_defs.t9,
        }
    }
    /*
    int ,,x7,y7,i,j,;
    double x3,y3,n,rn,h,;
    char ans,fbuff[81],[7]es[16],cmdbuff[8];
     */

    /// Repair anything that is down
    pub fn fix_damage(&mut self) {
        for i in 0..6 {
            self.damage[i] = 0;
        }
    } /* End fixdamage */

    /// Initialize
    pub fn init<W: WriteColor>(&mut self, sout: &mut W) -> StResult<()> {
        rand_init();
        self.fix_damage();
        let (mut x, mut y) = set_random_x_y();
        self.set_current_quadrant_from_coords(x, y);
        x = 8;
        y = 1;
        let mut b9 = self.total_starbases;

        let the_game_defs = self.game_defs;
        let mut t9 = the_game_defs.t9;
        let t0 = the_game_defs.t0;
        let mut k9 = self.total_klingons as i32;
        let x1 = self.game_defs.x1;
        let x2 = self.game_defs.x2;
        let y1 = self.game_defs.y1;
        let y2 = self.game_defs.y2;
        let mut k = self.k;

        for i in 0..8 {
            for j in 0..8 {
                k = 0;
                let mut n = rnd();
                if n < x1 {
                    n *= 64.0;
                    k = lt(n, y1) as i32 - (y as i32);
                    k = k
                        + lt(n, x2) as i32
                        + lt(n, y2) as i32
                        + lt(n, 0.08) as i32
                        + lt(n, 0.03) as i32
                        + lt(n, 0.01) as i32;
                    k9 -= k as i32;
                }
                self.b = gt(rnd(), self.game_defs.aa);
                b9 -= self.b as i32; //self.cas f64 self.was i32
                let quadrant = Quadrant::new(i, j);
                self.quad[quadrant] = QuadrantContents::from_i32(
                    (k * 100 + (self.b * 10)) - (rnd() * (x as f64) + (y as f64)).floor() as i32,
                );
            }
        }

        if k9 > (t9 - t0) as i32 {
            t9 = t0 + k9 as i32;
        }

        if b9 <= 0 {
            let (starbase_x, starbase_y) = set_random_x_y();
            let quadrant = Quadrant::new(starbase_x, starbase_y);
            let mut quadrant_value = self.quad[quadrant].as_i32();
            debug!(
                "About to subtract ten from quadrant {} with value {}",
                quadrant, quadrant_value
            );
            quadrant_value -= 10;
            debug!(
                "About to store value {} in quadrant {}",
                quadrant_value, quadrant
            );
            self.quad[quadrant] = QuadrantContents::from_i32(quadrant_value);
            b9 = 1;
        }

        self.k = k;
        self.total_klingons = k9 as i32;
        self.k0 = k9 as i32;
        self.total_starbases = b9;
        self.years = (t9 - t0) as i32;
        self.ending_stardate = t9;
        writeln!(
            sout,
            "OBJECTIVE: DESTROY {} KLINGON BATTLE CRUISERS IN {} YEARS.",
            self.total_klingons, self.years
        )?;
        writeln!(sout, " THE NUMBER OF STARBASES IS {}.\n", b9)?;

        Ok(())
    } /* End init */

    pub fn increment_year(&mut self) {
        self.years -= 1;
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
        let e0 = self.game_defs.e0;
        let p0 = self.game_defs.p0;
        for i in (s1 - 1)..=(s1 + 1) {
            for j in (s2 - 1)..=(s2 + 1) {
                if (i >= 0) && (i <= 7) && (j >= 0) && (j <= 7) {
                    let sector = Sector::new(i as i32, j as i32);
                    if self.sect[sector] == SectorContents::Starbase.into() {
                        // Docked at starbase
                        self.cond = Condition::Docked;
                        self.e = e0;
                        self.p = p0;
                        self.fix_damage();
                        return;
                    }
                }
            }
        }
        if self.k > 0 {
            // Klingons present!
            self.cond = Condition::Red;
        } else if self.e < (0.1 * e0) {
            // Low energy
            self.cond = Condition::Yellow;
        } else {
            // A-OK!
            self.cond = Condition::Green;
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
        .map_err(|e| {
            let e = e.into();
            e
        })
    } /* End showhit */

    /// Show estimated time for repair
    fn show_est_repair_time<W: WriteColor>(&self, sout: &mut W, i: usize) -> StResult<()> {
        writeln!(sout, "{} YEARS ESTIMATED FOR REPAIR.\n", self.damage[i]).map_err(|e| {
            let e = e.into();
            e
        })
    } /* End showestreptime */

    /// Show damaged item
    fn show_damage<W: WriteColor>(&self, sout: &mut W, i: usize) -> StResult<()> {
        write!(sout, "{} DAMAGED.  ", DS[i])?;
        sout.flush()?;
        beep();
        self.show_est_repair_time(sout, i)
    } /* End showdamage */

    fn is_docked(&self) -> bool {
        // This is an amazingly stupid way to do this, but it's how they do it
        self.cond == Condition::Docked
    }

    /// Set up string for lr scan or galactic records
    fn qstr<W: WriteColor>(&self, sout: &mut W, i: i32, j: i32, is_current: bool) -> StResult<()> {
        let quadrant = Quadrant::new(i, j);
        // The printf format string was "%3.3i", which has a width of 3 digits and has leading 0s.
        // I _think_.
        let value = self.quad[quadrant];
        let emphasize = is_current;
        value.draw(sout, emphasize)?;
        Ok(())
    } /* End qstr */

    /// Check for hits from Klingons
    fn check_for_hits<W: WriteColor>(&mut self, sout: &mut W) -> StResult<()> {
        if self.k < 1 {
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
                self.e -= h;
                let n: f64 = self.e;
                self.show_hit(sout, i, "ENTERPRISE FROM", n, h)?;
            }
        }
        Ok(())
    } /* End checkforhits */

    /// Do long-range scan
    fn l_range_scan<W: WriteColor>(&mut self, sout: &mut W) -> StResult<()> {
        let i = 2;
        if self.damage[i] > 0 {
            // Long-range scan inoperative
            self.show_damage(sout, i)?;
            return Ok(());
        }
        let q1: i32 = self.q1 as i32;
        let q2: i32 = self.q2 as i32;
        writeln!(sout, "{} FOR QUADRANT {} - {}", DS[i], q1 + 1, q2 + 1)?;
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
                    self.quad[quadrant].show();
                    self.qstr(sout, i as i32, j as i32, self.is_current_quadrant(i, j))?;
                }
            }
            writeln!(sout)?;
        }
        Ok(())
    } /* End lrscan */

    /// Do galactic records
    fn galactic_records<W: WriteColor>(&self, sout: &mut W) -> StResult<()> {
        let i = 5;
        if self.damage[i] > 0 {
            // Galactic records inoperative
            self.show_damage(sout, i)?;
            return Ok(());
        }
        writeln!(
            sout,
            "CUMULATIVE GALACTIC MAP FOR STARDATE {}",
            self.current_stardate
        )?;
        for i in 0..8 {
            for j in 0..8 {
                write!(sout, "  ")?;
                sout.flush()?;
                self.qstr(sout, i as i32, j as i32, self.is_current_quadrant(i, j))?;
            }
            writeln!(sout)?;
        }
        Ok(())
    } /* End galrecs */

    /// Do short-range scan
    fn s_range_scan<W: WriteColor>(&mut self, sout: &mut W, a: i32) -> StResult<()> {
        self.check_condition(); //?
        if a == 0
        /* Initial entry into quadrant */
        {
            self.check_for_hits(sout)?;
            if self.e <= 0.0 {
                /* Ran out of energy! */
                return Ok(());
            }
        }
        let i = 1;
        if self.damage[i] > 0 {
            // Short-range scan inoperative
            self.show_damage(sout, i)?;
            return Ok(());
        }
        for i in 0..8 {
            for j in 0..8 {
                write!(sout, "{} ", self.sect.sector_char_at_coords(i, j))?;
                sout.flush()?;
            }
            write!(sout, "  ")?;
            sout.flush()?;
            match i {
                0 => {
                    writeln!(
                        sout,
                        "YEARS = {}",
                        self.game_defs.t9 - self.current_stardate
                    )?;
                }
                1 => {
                    writeln!(sout, "STARDATE = {}", self.current_stardate)?;
                }
                2 => {
                    write!(sout, "CONDITION: ")?;
                    sout.set_color(&self.cond.get_color_spec())?;
                    writeln!(sout, "{}", self.cond.as_ref())?;
                    sout.reset()?;
                }
                3 => {
                    writeln!(sout, "QUADRANT = {} - {}", self.q1 + 1, self.q2 + 1)?;
                }
                4 => {
                    writeln!(sout, "SECTOR = {} - {}", self.s1 + 1, self.s2 + 1)?;
                }
                5 => {
                    writeln!(sout, "ENERGY = {:03}", self.e)?; // printf format string was "%.3f"
                }
                6 => {
                    writeln!(sout, "{} = {}", DS[4], self.p)?;
                }
                7 => {
                    writeln!(sout, "KLINGONS LEFT = {}", self.total_klingons)?;
                }
                _ => {}
            }
        }
        Ok(())
    } /* End srscan */

    /// Fire phasers
    fn phasers<R: BufRead, W: WriteColor>(&mut self, sin: &mut R, sout: &mut W) -> StResult<f64> {
        let mut x = 0.0;
        let i = 3;
        if self.damage[i] > 0 {
            // Phasers inoperative
            self.show_damage(sout, i)?;
            return Ok(x);
        }
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
            if x <= self.e {
                break;
            }
            writeln!(sout, "ONLY GOT {:03}", self.e)?; // The printf format was "%.3f"
        }
        self.e -= x;
        let y3 = self.k as f64;
        for i in 0..8 {
            if self.k3[i] > 0.0 {
                let f = fnd(self.k1[i], self.k2[i], self.s1, self.s2);
                debug!("About to fire phasers: x = {}, y3 = {}, f = {}", x, y3, f);
                let h = x / (y3 * f.powf(0.4));
                self.k3[i] -= h;
                let n = self.k3[i];
                self.show_hit(sout, i, "KLINGON AT", n, h)?;
                if self.k3[i] <= 0.0 {
                    writeln!(sout, "**KLINGON DESTROYED**")?;
                    self.k -= 1;
                    self.total_klingons -= 1;
                    let sector = Sector::new(self.k1[i], self.k2[i]);
                    self.sect[sector] = 1;
                    let quadrant = self.current_quadrant();
                    self.quad[quadrant].decrement_klingons();
                }
            }
        }
        Ok(x)
    } /* End phasers */

    /// Do the path for warp or torpedo
    fn do_path<W: WriteColor>(&mut self, sout: &mut W, a: Command, n: f64) -> StResult<()> {
        let mut y1 = self.s1 as f64 + 0.5;
        let mut x1 = self.s2 as f64 + 0.5;
        let mut y3 = (self.c - 1.0) as f64 * FRAC_PI_4; // `FRAC_PI_4` _was_ `0.785398`
        let x3 = y3.cos();
        y3 = -(y3.sin());
        let mut inquad = true;
        let mut shortmove = a == Command::WarpEngines; // Command #1
        let mut y7 = 0;
        let mut x7 = 0;
        let mut y2 = self.game_defs.y2;
        let mut x2 = self.game_defs.x2;
        for _ in 0..(n as usize) {
            y1 += y3;
            x1 += x3;
            y2 = y1.floor();
            x2 = x1.floor();
            y7 = y2 as i32;
            x7 = x2 as i32;
            if (x7 < 0) || (x7 > 7) || (y7 < 0) || (y7 > 7) {
                inquad = false;
                shortmove = false;
                break;
            }
            if a == Command::PhotonTorpedos
            // Command #5
            {
                // Show torpedo track
                write!(sout, "{} - {}  ", y7 + 1, x7 + 1)?;
                sout.flush()?;
            }
            if self.sect.sector_contents_at_coords(y7 as i32, x7 as i32) != SectorContents::Empty
            // Content type 1
            {
                // Object blocking move or hit by torpedo
                shortmove = false;
                break;
            }
        }

        if inquad {
            // Still in quadrant -- short move, block, or torpedo hit
            self.newquad = false;
            writeln!(sout)?;
            if !shortmove {
                if a == Command::WarpEngines
                // Comman #1
                {
                    write!(sout, "BLOCKED BY ")?;
                    sout.flush()?;
                }
                match self.sect.sector_contents_at_coords(y7 as i32, x7 as i32) {
                    SectorContents::Klingon => {
                        // case 3 :
                        // Klingon
                        write!(sout, "KLINGON")?;
                        sout.flush()?;
                        if a == Command::PhotonTorpedos
                        // Command #5
                        {
                            // Torpedo
                            for i in 0..8 {
                                if (y7 == self.k1[i] as i32) && (x7 == self.k2[i] as i32) {
                                    self.k3[i] = 0.0;
                                }
                            }
                            self.k -= 1;
                            self.total_klingons -= 1;
                        }
                    }
                    SectorContents::Starbase => {
                        // case 4 :
                        // Starbase
                        write!(sout, "STARBASE")?;
                        sout.flush()?;
                        if a == Command::PhotonTorpedos
                        // Command #5
                        {
                            // Torpedo
                            self.b = 2;
                        }
                    }
                    SectorContents::Star => {
                        // case 5 :
                        // Star
                        write!(sout, "STAR")?;
                        sout.flush()?;
                        if a == Command::PhotonTorpedos
                        // Command #5
                        {
                            // Torpedo
                            self.s -= 1;
                        }
                    }
                    _ => {
                        return Err(StarTrustError::GameStateError(format!(
                            "Ship blocked by unknown object"
                        )))
                    }
                }
                if a == Command::WarpEngines
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
            if a == Command::WarpEngines
            // Command #1
            {
                self.s1 = y2 as i32;
                self.s2 = x2 as i32;
                let the_sector = self.current_sector();
                self.sect[the_sector] = 2;
                // Flag to show we stayed within quadrant
                self.saved_command = 2.into();
            } else if a == Command::PhotonTorpedos
            // Command #5
            {
                // Torpedo
                write!(sout, " DESTROYED!")?;
                sout.flush()?;
                if self.b == 2 {
                    self.b = 0;
                    write!(sout, " . . . GOOD WORK!")?;
                    sout.flush()?;
                }
                writeln!(sout)?;
                let old_sector = Sector::new(y7 as i32, x7 as i32);
                self.sect[old_sector] = SectorContents::Empty.into(); // Clear old sector (set it to 1)
                let current_quadrant = Quadrant::new(self.q1, self.q2);
                self.quad[current_quadrant] = QuadrantContents::new(self.k, self.b, self.s, false);
            }
        } else {
            // Out of quadrant -- move to new quadrant or torpedo miss
            if a == Command::WarpEngines
            // Command #1
            {
                // Move
                self.newquad = true;
                self.q1 =
                    (self.q1 as f64 + self.w * y3 + (self.s1 as f64 + 0.5) / 8.0).floor() as i32; //u8
                self.q2 =
                    (self.q2 as f64 + self.w * x3 + (self.s2 as f64 + 0.5) / 8.0).floor() as i32; //u8
                self.q1 =
                    (self.q1 as i32 - lt(self.q1 as f64, 0.0) + gt(self.q1 as f64, 7.0)) as i32; //u8
                self.q2 =
                    (self.q2 as i32 - lt(self.q2 as f64, 0.0) + gt(self.q2 as f64, 7.0)) as i32;
            //u8
            } else if a == Command::PhotonTorpedos
            // Command #5
            {
                // Torpedo
                writeln!(sout, "MISSED!")?;
            }
        }
        Ok(())
    } /* End dopath */

    pub fn do_warp<R: BufRead, W: WriteColor>(
        &mut self,
        sin: &mut R,
        sout: &mut W,
        a: &mut Command,
        gamecomp: &mut GameState,
        moved: &mut bool,
    ) -> StResult<()> {
        let mut w = 0f64;
        let mut c; // = self.c
        loop {
            loop {
                c = getcourse(sin, sout)?; // self
                self.c = c;
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
                    if (self.damage[0] > 0) && (w > 0.2) {
                        let i = 0;
                        write!(sout, "{} DAMAGED; MAX IS 0.2; ", DS[i])?;
                        sout.flush()?;
                        self.show_est_repair_time(sout, i)?;
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
        self.check_for_hits(sout)?;
        if self.e <= 0.0 {
            /* Ran out of energy */
            *gamecomp = (-1).into();
            return Ok(());
        }

        if rnd() <= 0.25 {
            let x = (rnd() * 6.0).floor() as usize;
            if rnd() <= 0.5 {
                beep();
                self.damage[x] += (6.0 - rnd() * 5.0).floor() as i32;
                writeln!(sout, "**SPACE STORM, {} DAMAGED**", DS[x])?;
                let i = x;
                self.show_est_repair_time(sout, i)?;
                self.damage[x] += 1;
                delay(100);
                beep();
            } else {
                let mut j: i32 = -1;
                for i in x..6 {
                    if self.damage[i] > 0 {
                        j = i as i32;
                        break;
                    }
                }
                if j < 0 {
                    for i in 0..x {
                        if self.damage[i] > 0 {
                            j = i as i32;
                            break;
                        }
                    }
                }
                if j >= 0 {
                    self.damage[j as usize] = 1;
                    writeln!(sout, "**SPOCK USED A NEW REPAIR TECHNIQUE**")?;
                }
            }
        }
        for i in 0..6 {
            if self.damage[i] != 0 {
                self.damage[i] -= 1;
                if self.damage[i] <= 0 {
                    self.damage[i] = 0;
                    writeln!(sout, "{} ARE FIXED!", DS[i])?;
                    beep();
                }
            }
        }
        let n = (w * 8.0).floor();
        self.w = w;
        self.e = self.e - n - n + 0.5;
        self.current_stardate += 1i32;
        let current_sector = self.current_sector();
        self.sect[current_sector] = 1;
        if self.current_stardate > self.game_defs.t9 {
            /* Ran out of time! */
            *gamecomp = (-1).into();
            return Ok(());
        }
        self.do_path(sout, *a, n)?;
        *a = self.saved_command;
        // let i = n;
        if self.e <= 0.0 {
            // Ran out of energy
            *gamecomp = (-1).into();
            return Ok(());
        }
        *moved = true;
        Ok(())
    }

    fn do_torpedoes<R: BufRead, W: WriteColor>(
        &mut self,
        sin: &mut R,
        sout: &mut W,
        a: &mut Command,
        gamecomp: &mut GameState,
    ) -> StResult<()> {
        if self.damage[4] > 0 {
            // Torpedoes damaged
            write!(sout, "SPACE CRUD BLOCKING TUBES.  ")?;
            sout.flush()?;
            let i = 4;
            self.show_est_repair_time(sout, i)?;
            beep();
            return Ok(());
        }
        let n: f64 = 15.0;
        if self.p < 1 {
            writeln!(sout, "NO TORPEDOES LEFT!")?;
            return Ok(());
        }
        self.c = 10.0;
        while self.c >= 9.0 {
            write!(sout, "TORPEDO ")?;
            sout.flush()?;

            self.c = getcourse(sin, sout)?; // self
        }
        if self.c < 1.0 {
            // Abort firing of torpedo
            return Ok(());
        }
        self.p -= 1;
        write!(sout, "TRACK: ")?;
        sout.flush()?;
        self.do_path(sout, *a, n)?;
        *a = self.saved_command;
        // let i = n;
        if self.e <= 0.0 {
            /* Ran out of energy */
            *gamecomp = (-1).into();
        }
        self.check_for_hits(sout)?;
        if self.e <= 0.0 {
            /* Ran out of energy */
            *gamecomp = (-1).into();
        }
        if self.total_klingons < 1 {
            /* All Klingons destroyed! */
            *gamecomp = 1.into();
        }
        if !gamecomp.is_done() {
            self.check_condition();
        }
        Ok(())
    }

    pub fn s9(&self) -> f64 {
        self.game_defs.s9
    }

    pub fn play<R: BufRead, W: WriteColor>(&mut self, sin: &mut R, sout: &mut W) -> StResult<()> {
        let mut gamecomp = GameState::InProgress;
        let mut moved: bool = false;
        let mut a = self.saved_command;

        debug!("Init gamecomp={:?}, moved={}, a={:?}", gamecomp, moved, a);
        self.init(sout)?;
        self.newquad = true;
        debug!(
            "Done initing gamecomp={:?}, moved={}, a={:?}, newquad={}",
            gamecomp, moved, a, self.newquad
        );

        while !gamecomp.is_done() {
            if self.newquad {
                setup_quadrant(self);
                a = self.saved_command;
            }
            self.newquad = false;
            moved = false;
            self.s_range_scan(sout, a.into())?;
            if self.e <= 0.0 {
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
                    let int_a;
                    match ebuff {
                        InputValue::Blank => int_a = 99,
                        InputValue::Esc => int_a = -99,
                        InputValue::InputString(cmdbuff) => {
                            if let Ok(cmd) = cmdbuff.parse::<i32>() {
                                int_a = cmd;
                            } else {
                                continue;
                            }
                        }
                    }
                    if int_a == -99 {
                        write!(sout, "\nARE YOU SURE YOU WANT TO QUIT? ")?;
                        sout.flush()?;
                        let ans = yesno(sin)?; // sout
                        if ans == 'Y' {
                            gamecomp = (-99).into();
                            break; /* Break out of command loop */
                        } else {
                            continue;
                        } /* Back to top of command loop */
                    } else if (int_a < 1) || (int_a > 6) {
                        for i in 0..6 {
                            writeln!(sout, "  {} = {}", i + 1, DS[i])?;
                        }
                        writeln!(sout, "  -99 OR ESC TO QUIT\n")?;
                        // Back to top of command loop
                        continue;
                    } else {
                        a = int_a.into()
                    }
                    match a {
                        Command::WarpEngines => {
                            //case 1 :
                            // Warp engines
                            self.do_warp(sin, sout, &mut a, &mut gamecomp, &mut moved)?;
                        }
                        Command::ShortRangeScan => {
                            //case 2 :
                            // Short-range scan
                            self.s_range_scan(sout, a.into())?;
                        }
                        Command::LongRangeScan => {
                            //case 3 :
                            /* Long-range scan */
                            self.l_range_scan(sout)?;
                        }
                        Command::Phasers => {
                            //case 4 :
                            /* Phasers */
                            let x = self.phasers(sin, sout)?;
                            if x > 0.0 {
                                if self.e <= 0.0 {
                                    /* Ran out of energy */
                                    gamecomp = (-1).into();
                                }
                                self.check_for_hits(sout)?;
                                if self.e <= 0.0 {
                                    /* Ran out of energy */
                                    gamecomp = (-1).into();
                                }
                                if self.total_klingons < 1 {
                                    /* All Klingons destroyed! */
                                    gamecomp = 1.into();
                                }
                                if !gamecomp.is_done() {
                                    self.check_condition()
                                };
                            }
                        }
                        Command::PhotonTorpedos => {
                            //case 5 :
                            // Photon torpedoes
                            self.do_torpedoes(sin, sout, &mut a, &mut gamecomp)?;
                        }
                        Command::Galacticrecords => {
                            //case 6 :
                            /* Galactic records */
                            self.galactic_records(sout)?;
                        }
                        Command::Undefined => {
                            error!("undefined command in command loop.")
                        }
                        Command::Quit => {
                            gamecomp = GameState::Quit;
                            break;
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
                let t0 = self.game_defs.t0;
                let drate: f64 = (t - t0) as f64;
                let rating: i32 = ((self.k0 as f64 / drate) * 1000.0) as i32;
                writeln!(sout, "THE FEDERATION HAS BEEN SAVED!")?;
                writeln!(sout, "YOU ARE PROMOTED TO ADMIRAL.")?;
                writeln!(
                    sout,
                    "{} KLINGONS IN {} YEARS.  RATING = {}\n",
                    self.k0,
                    t - t0,
                    rating,
                )?;
            }
            GameState::Lost => {
                if self.current_stardate > self.game_defs.t9 {
                    writeln!(sout, "YOU RAN OUT OF TIME!")?;
                } else if self.e <= 0.0 {
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
