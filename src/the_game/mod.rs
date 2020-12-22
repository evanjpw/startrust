//! # startrust::the_game

use std::io::{BufRead, Write};

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::error::StarTrustError::GameStateError;
use crate::interaction::{beep, getinp, InputMode, InputValue};
use crate::the_game::commands::Command;
pub use crate::the_game::config::{TheGameDefs, TheGameDefsBuilder};
use crate::the_game::quadrant::{Quadrant, QuadrantMap};
pub use crate::the_game::sector::{Sector, SectorContents, SectorMap};
use crate::the_game::stardate::StarDate;
use crate::util::{findslot, fnd, gt, lt, randinit, rnd, setrndxy};
use crate::{StResult, StarTrustError};
use std::f64::consts::FRAC_PI_4;

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

pub struct TheGame {
    /// Current Energy
    e: f64,
    /// Current Photon Torpedoes
    p: u16,
    /// Current StarDate
    t: StarDate,
    years: u8,
    /// Total remaining Klingons
    k9: u16,
    /// The Enterprise's x position within the Quadrant
    s1: u8,
    /// The Enterprise's y position within the Quadrant
    s2: u8,
    /// The x position of the current Quadrant
    q1: u8,
    /// The y position of the current Quadrant
    q2: u8,
    /// The Damage Array
    d: Vec<i32>,
    k0: u16,
    k1: Vec<u8>,
    k2: Vec<u8>,
    k3: Vec<f64>,
    /// The number of Starbases
    b9: u16,
    /// New Quadrant
    newquad: bool,
    k: i16,
    /// The Sector Map
    pub(crate) sect: SectorMap,
    /// The Quadrant Map
    pub(crate) quad: QuadrantMap,
    pub(crate) game_defs: TheGameDefs,
    c: f64,
    /// Warp
    w: f64,
    b: u32,
    /// The current condition of the Enterprise as a String
    cond: &'static str,
    saved_command: Command,
}

#[derive(Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(i32)]
enum GameState {
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
            t: the_game_defs.t0,
            years: (the_game_defs.t9 - the_game_defs.t0) as u8,
            k9: the_game_defs.k9,
            sect: SectorMap::new(),
            quad: QuadrantMap::new(),
            s1: 0,
            s2: 0,
            q1: 0,
            q2: 0,
            d: vec![0i32; 6],
            k0: 0,
            k1: vec![0u8; 8],
            k2: vec![0u8; 8],
            k3: vec![0.0; 8],
            game_defs: *the_game_defs,
            b9,
            newquad: false,
            k: 0,
            c,
            w,
            b: 0,
            cond: "",
            saved_command: Command::Undefined,  // the global version of `a`
        }
    }
    /*
    int ,,x7,y7,i,j,s,;
    double x3,y3,n,rn,h,;
    char ans,fbuff[81],[7]es[16],cmdbuff[8];
     */

    /// Repair anything that is down
    pub fn fixdamage(&mut self) {
        for i in 0..6 {
            self.d[i] = 0;
        }
    } /* End fixdamage */

    /// Initialize
    pub fn init<W: Write>(&mut self, sout: &mut W) -> StResult<()> {
        randinit();
        self.fixdamage();
        let (x, y) = setrndxy();
        self.set_current_sector_from_coords(x, y);
        let mut b9 = self.b9;

        let the_game_defs = self.game_defs;
        let mut t9 = the_game_defs.t9;
        let t0 = the_game_defs.t0;
        let mut k9 = self.k9;
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
                    k = lt(n, y1) as i16 - (y as i16);
                    k = k
                        + lt(n, x2) as i16
                        + lt(n, y2) as i16
                        + lt(n, 0.08) as i16
                        + lt(n, 0.03) as i16
                        + lt(n, 0.01) as i16;
                    k9 -= k as u16;
                }
                self.b = gt(rnd(), self.game_defs.aa) as u32;
                b9 -= self.b as u16;
                let quadrant = Quadrant::new(i, j);
                self.quad[quadrant] = (k as f64 * self.c + (self.b as f64) * self.w
                    - (rnd() * (x as f64) + (y as f64)).floor())
                    as i16;
            }
        }

        if k9 > (t9 - t0) {
            t9 = t0 + k9;
        }

        if b9 <= 0 {
            let (starbase_x, starbase_y) = setrndxy();
            let quadrant = Quadrant::new(starbase_x, starbase_y);
            self.quad[quadrant] -= 10;
            b9 = 1;
        }

        self.k = k;
        self.k9 = k9;
        self.k0 = k9;
        self.b9 = b9;
        self.years = (t9 - t0) as u8;
        writeln!(
            sout,
            "OBJECTIVE: DESTROY {} KLINGON BATTLE CRUISERS IN {} YEARS.",
            self.k9, self.years
        )?;
        writeln!(sout, " THE NUMBER OF STARBASES IS {}.\n", b9)?;

        Ok(())
    } /* End init */

    pub fn increment_year(&mut self) {
        self.years -= 1;
        self.t += 1u16;
    }

    fn current_sector(&self) -> Sector {
        Sector::new(self.s1, self.s2)
    }

    fn set_current_sector(&mut self, sector: Sector) {
        self.set_current_quadrant_from_coords(sector.x(), sector.y())
    }

    fn set_current_sector_from_coords(&mut self, x: u8, y: u8) {
        self.s1 = x;
        self.s2 = y;
    }

    fn current_quadrant(&self) -> Quadrant {
        Quadrant::new(self.q1, self.q2)
    }

    fn set_current_quadrant(&mut self, quadrant: Quadrant) {
        self.set_current_quadrant_from_coords(quadrant.x(), quadrant.y());
    }

    fn set_current_quadrant_from_coords(&mut self, x: u8, y: u8) {
        self.q1 = x;
        self.q2 = y;
    }

    /// Display current star date
    pub fn showstardate<W: Write>(&self, sout: &mut W) -> StResult<()> {
        write!(sout, "\nIT IS STARDATE {}.\n", self.t).map_err(|e| {
            let e: StarTrustError = e.into();
            e
        })
    } /* End showstardate */

    /// Check condition
    fn checkcond(&mut self) {
        let s1 = self.s1 as i32;
        let s2 = self.s2 as i32;
        let e0 = self.game_defs.e0;
        let p0 = self.game_defs.p0;
        for i in (s1 - 1)..=(s1 + 1) {
            for j in (s2 - 1)..=(s2 + 1) {
                if (i >= 0) && (i <= 7) && (j >= 0) && (j <= 7) {
                    let sector = Sector::new(i as u8, j as u8);
                    if self.sect[sector] == SectorContents::Starbase.into() {
                        // Docked at starbase
                        self.cond = "DOCKED";
                        self.e = e0;
                        self.p = p0;
                        self.fixdamage();
                        return;
                    }
                }
            }
        }
        if self.k > 0 {
            // Klingons present!
            self.cond = "RED";
        } else if self.e < (0.1 * e0) {
            // Low energy
            self.cond = "YELLOW";
        } else {
            // A-OK!
            self.cond = "GREEN";
        }
    } /* End checkcond */

    /// Show hit on Enterprise or Klingon
    fn showhit<W: Write>(&self, sout: &mut W, i: usize, es: &str, n: f64, h: f64) -> StResult<()> {
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
    fn showestreptime<W: Write>(&self, sout: &mut W, i: usize) -> StResult<()> {
        writeln!(sout, "{} YEARS ESTIMATED FOR REPAIR.\n", self.d[i]).map_err(|e| {
            let e = e.into();
            e
        })
    } /* End showestreptime */

    /// Show damaged item
    fn showdamage<W: Write>(&self, sout: &mut W, i: usize) -> StResult<()> {
        write!(sout, "{} DAMAGED.  ", DS[i]).map_err(|e| {
            let e: StarTrustError = e.into();
            e
        })?;
        beep();
        self.showestreptime(sout, i)
    } /* End showdamage */

    fn is_docked(&self) -> bool {
        // This is an amazingly stupid way to do this, but it's how they do it
        self.cond == "DOCKED"
    }

    /// Set up string for lr scan or galactic records
    fn qstr(&self, i: u8, j: u8) -> String {
        let quadrant = Quadrant::new(i, j);
        // The printf format string was "%3.3i", which has a width of 3 digits and has leading 0s.
        // I _think_.
        format!("{:03}", self.quad[quadrant])
    } /* End qstr */

    /// Check for hits from Klingons
    fn checkforhits<W: Write>(&mut self, sout: &mut W) -> StResult<()> {
        if self.k < 1 {
            /* No Klingons here! */
            return Ok(());
        }
        if self.is_docked() {
            writeln!(sout, "STARBASE PROTECTS ENTERPRISE.").map_err(|e| {
                let e: StarTrustError = e.into();
                e
            })?;
            return Ok(());
        }
        for i in 0..8 {
            if self.k3[i] > 0.0 {
                let mut h = self.k3[i] * 0.4 * rnd();
                self.k3[i] -= h;
                h /= fnd(self.k1[i], self.k2[i], self.s1, self.s2).powf(0.4);
                self.e -= h;
                let n: f64 = self.e;
                self.showhit(sout, i, "ENTERPRISE FROM", n, h)?;
            }
        }
        Ok(())
    } /* End checkforhits */

    /// Do long-range scan
    fn lrscan<W: Write>(&mut self, sout: &mut W) -> StResult<()> {
        let i = 2;
        if self.d[i] > 0 {
            // Long-range scan inoperative
            self.showdamage(sout, i)?;
            return Ok(());
        }
        let q1: i32 = self.q1 as i32;
        let q2: i32 = self.q2 as i32;
        writeln!(sout, "{} FOR QUADRANT {} - {}", DS[i], q1 + 1, q2 + 1).map_err(|e| {
            let e: StarTrustError = e.into();
            e
        })?;
        for i in (q1 - 1)..=(q1 + 1) {
            for j in (q2 - 1)..=(q2 + 1) {
                write!(sout, "   ").map_err(|e| {
                    let e: StarTrustError = e.into();
                    e
                })?;
                if (i < 0) || (i > 7) || (j < 0) || (j > 7) {
                    write!(sout, "***").map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                } else {
                    let quadrant = Quadrant::new(i as u8, j as u8);
                    let value = self.quad[quadrant].abs();
                    self.quad[quadrant] = value;
                    let es = self.qstr(i as u8, j as u8);
                    write!(sout, "{}", es).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                }
            }
            writeln!(sout).map_err(|e| {
                let e: StarTrustError = e.into();
                e
            })?;
        }
        Ok(())
    } /* End lrscan */

    /// Do galactic records
    fn galrecs<W: Write>(&self, sout: &mut W) -> StResult<()> {
        let i = 5;
        if self.d[i] > 0 {
            // Galactic records inoperative
            self.showdamage(sout, i)?;
            return Ok(());
        }
        writeln!(sout, "CUMULATIVE GALACTIC MAP FOR STARDATE {}", self.t).map_err(|e| {
            let e: StarTrustError = e.into();
            e
        })?;
        for i in 0..8 {
            for j in 0..8 {
                write!(sout, "  ").map_err(|e| {
                    let e: StarTrustError = e.into();
                    e
                })?;
                let quadrant = Quadrant::new(i, j);
                if self.quad[quadrant] < 0 {
                    write!(sout, "***").map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                } else {
                    let es = self.qstr(i as u8, j as u8);
                    write!(sout, "{}", es).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                }
            }
            writeln!(sout).map_err(|e| {
                let e: StarTrustError = e.into();
                e
            })?;
        }
        Ok(())
    } /* End galrecs */

    /// Do short-range scan
    fn srscan<W: Write>(&mut self, sout: &mut W, a: i32) -> StResult<()> {
        self.checkcond(); //?
        if a == 0
        /* Initial entry into quadrant */
        {
            self.checkforhits(sout)?;
            if (self.e <= 0.0) {
                /* Ran out of energy! */
                return Ok(());
            }
        }
        let i = 1;
        if self.d[i] > 0 {
            // Short-range scan inoperative
            self.showdamage(sout, i)?;
            return Ok(());
        }
        for i in 0..8 {
            for j in 0..8 {
                write!(sout, "{} ", self.sect.sector_char_at_coords(i, j)).map_err(|e| {
                    let e: StarTrustError = e.into();
                    e
                })?;
            }
            write!(sout, "  ").map_err(|e| {
                let e: StarTrustError = e.into();
                e
            })?;
            match i {
                0 => {
                    writeln!(sout, "YEARS = {}", self.game_defs.t9 - self.t).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                    // break;
                }
                1 => {
                    writeln!(sout, "STARDATE = {}", self.t).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                    // break;
                }
                2 => {
                    writeln!(sout, "CONDITION: {}", self.cond).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                    // break;
                }
                3 => {
                    writeln!(sout, "QUADRANT = {} - {}", self.q1 + 1, self.q2 + 1).map_err(
                        |e| {
                            let e: StarTrustError = e.into();
                            e
                        },
                    )?;
                    // break;
                }
                4 => {
                    writeln!(sout, "SECTOR = {} - {}", self.s1 + 1, self.s2 + 1).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                    // break;
                }
                5 => {
                    writeln!(sout, "ENERGY = {:03}", self.e) // printf format string was "%.3f"
                        .map_err(|e| {
                            let e: StarTrustError = e.into();
                            e
                        })?;
                    // break;
                }
                6 => {
                    writeln!(sout, "{} = {}", DS[4], self.p).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                    // break;
                }
                7 => {
                    writeln!(sout, "KLINGONS LEFT = {}", self.k9).map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                    // break;
                }
                _ => {}
            }
        }
        Ok(())
    } /* End srscan */

    /// Fire phasers
    fn phasers<W: Write>(&mut self, sout: &mut W) -> StResult<()> {
        let mut x = 0.0;
        let i = 3;
        if self.d[i] > 0 {
            // Phasers inoperative
            self.showdamage(sout, i)?;
            return Ok(());
        }
        loop {
            write!(sout, "PHASERS READY: ENERGY UNITS TO FIRE? ").map_err(|e| {
                let e: StarTrustError = e.into();
                e
            })?;
            let gb = getinp(15, InputMode::Mode2);
            writeln!(sout).map_err(|e| {
                let e: StarTrustError = e.into();
                e
            })?;
            if let InputValue::InputString(ibuff) = gb {
                x = ibuff.parse()?;
            } else {
                x = 0.0;
                break;
            }
            if x <= self.e {
                break;
            }
            writeln!(sout, "ONLY GOT {:03}", self.e) // The printf format was "%.3f"
                .map_err(|e| {
                    let e: StarTrustError = e.into();
                    e
                })?;
        }
        self.e -= x;
        let y3 = self.k as f64;
        for i in 0..8 {
            if self.k3[i] > 0.0 {
                let f = fnd(self.k1[i], self.k2[i], self.s1, self.s2);
                let h = x / (y3 * f.powf(0.4));
                self.k3[i] -= h;
                let n = self.k3[i];
                self.showhit(sout, i, "KLINGON AT", n, h)?;
                if self.k3[i] <= 0.0 {
                    writeln!(sout, "**KLINGON DESTROYED**").map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                    self.k -= 1;
                    self.k9 -= 1;
                    let sector = Sector::new(self.k1[i], self.k2[i]);
                    self.sect[sector] = 1;
                    let quadrant = self.current_quadrant();
                    self.quad[quadrant] -= 100;
                }
            }
        }
        Ok(())
    } /* End phasers */

    /// Do the path for warp or torpedo
    fn dopath<W: Write>(&mut self, sout: &mut W, a: Command, n: f64) -> StResult<()> {
        let mut y1 = self.s1 as f64 + 0.5;
        let mut x1 = self.s2 as f64 + 0.5;
        let mut y3 = (self.c - 1.0) as f64 * FRAC_PI_4; // `FRAC_PI_4` _was_ `0.785398`
        let x3 = y3.cos();
        y3 = -(y3.sin());
        let mut inquad = true;
        let mut shortmove = (a == Command::WarpEngines); // Command #1
        for i in 0..(n as usize) {
            y1 += y3;
            x1 += x3;
            let y2 = y1.floor();
            let x2 = x1.floor();
            let y7 = y2 as i32;
            let x7 = x2 as i32;
            if (x7 < 0) || (x7 > 7) || (y7 < 0) || (y7 > 7) {
                inquad = false;
                shortmove = false;
                break;
            }
            if a == Command::PhotonTorpedos
            // Command #5
            {
                /* Show torpedo track */
                write!(sout, "{} - {}  ", y7 + 1, x7 + 1).map_err(|e| {
                    let e: StarTrustError = e.into();
                    e
                })?;
            }
            if self.sect.sector_contents_at_coords(y7 as u8, x7 as u8) != SectorContents::Empty
            // Content type 1
            {
                /* Object blocking move or hit by torpedo */
                shortmove = false;
                break;
            }
        }

        if inquad {
            // Still in quadrant -- short move, block, or torpedo hit
            self.newquad = false;
            writeln!(sout).map_err(|e| {
                let e: StarTrustError = e.into();
                e
            })?;
            if !shortmove {
                if a == Command::WarpEngines
                // Comman #1
                {
                    write!(sout, "BLOCKED BY ").map_err(|e| {
                        let e: StarTrustError = e.into();
                        e
                    })?;
                }
                match self.sect.sector_contents_at_coords(y7, x7) {
                    SectorContents::Klingon => {
                        // case 3 :
                        /* Klingon */
                        // cprintf("KLINGON");
                        if a == Command::PhotonTorpedos
                        // Command #5
                        {
                            /* Torpedo */
                            // for (i=0;i<8;i++){
                            // if ((y7==k1[i])&&(x7==k2[i])) {k3[i]=0.0;}}
                            // k--;
                            // k9--;
                        }
                        // break;
                    }
                    SectorContents::Starbase => {
                        // case 4 :
                        /* Starbase */
                        // cprintf("STARBASE");
                        if a == Command::PhotonTorpedos
                        // Command #5
                        {
                            /* Torpedo */
                            b = 2;
                        }
                        // break;
                    }
                    SectorContents::Star => {
                        // case 5 :
                        /* Star */
                        write!(sout, "STAR")
                            .map_err(|e| {
                                let e: StarTrustError = e.into();
                                e
                            })?
                        ;
                        if a == Command::PhotonTorpedos
                        // Command #5
                        {
                            /* Torpedo */
                            s -= 1;
                        }
                        // break;
                    }
                    _ => {
                        return Err(StarTrustError::GameStateError(format!(
                            "Ship blocked by unknown object"
                        )))
                    }
                }
                if a == Command::WarpEngines
                // Command #1
                /* Enterprise move */
                {
                    //         cprintf(" AT SECTOR %i - %i\r\n",y7+1,x7+1);
                    //         y2=floor(y1-y3);
                    //         x2=floor(x1-x3);
                    //         y7=y2;
                    //         x7=x2;
                }
            }
            if a == Command::WarpEngines
            // Command #1
            {
                        s1=y2;
                        s2=x2;
                        sect[s1][s2]=2;
                // Flag to show we stayed within quadrant
                self.saved_command = 2.into();
            } else if a == Command::PhotonTorpedos
            // Command #5
            {
                /* Torpedo */
                        write!(sout, " DESTROYED!")
                            .map_err(|e| {
                                let e: StarTrustError = e.into();
                                e
                            })?
                        ;
                if b == 2 {
                    b = 0;
                                write!(sout, " . . . GOOD WORK!")
                                    .map_err(|e| {
                                        let e: StarTrustError = e.into();
                                        e
                                    })?
                                ;
                }
                        writeln!(sout)
                            .map_err(|e| {
                                let e: StarTrustError = e.into();
                                e
                            })?
                        ;
                //         sect[y7][x7]=1;
                //         quad[q1][q2]=k*100+b*10+s;
            }
        } else {
            // Out of quadrant -- move to new quadrant or torpedo miss
            if a == Command::WarpEngines
            // Command #1
            {
                // Move
                self.newquad = true;
                self.q1 =
                    (self.q1 as f64 + self.w * y3 + (self.s1 as f64 + 0.5) / 8.0).floor() as u8;
                self.q2 =
                    (self.q2 as f64 + self.w * x3 + (self.s2 as f64 + 0.5) / 8.0).floor() as u8;
                self.q1 =
                    (self.q1 as i32 - lt(self.q1 as f64, 0.0) + gt(self.q1 as f64, 7.0)) as u8;
                self.q2 =
                    (self.q2 as i32 - lt(self.q2 as f64, 0.0) + gt(self.q2 as f64, 7.0)) as u8;
            } else if a == Command::PhotonTorpedos
            // Command #5
            {
                // Torpedo
                writeln!(sout, "MISSED!")
                    .map_err(|e| {
                    let e: StarTrustError = e.into();
                    e
                })?
                ;
            }
        }
        Ok(())
    } /* End dopath */

    pub fn s9(&self) -> f64 {
        self.game_defs.s9
    }

    pub fn play<W: Write>(&mut self, sout: &mut W) -> StResult<()> {
        let mut gamecomp = GameState::InProgress;
        let mut moved: bool = false;

        self.init(sout)?;
        self.newquad = true;

        while !gamecomp.is_done() {
            /*
               if (newquad) setupquad();
                    newquad=FALSE;
                    moved=FALSE;
                    srscan();
                    if (e<=0.0)  /* Ran out of energy */
                       gamecomp=-1;
                    else
                    {
                       while (TRUE)  /* Command loop (-99 or ESC to quit) */
                       {
                          cprintf("COMMAND? ");
                          a=getinp(cmdbuff,7,2);
                          cprintf("\r\n");
                          if (a==1) a=99;
                          if (a==-1) a=-99;
                          if (a==0) a=atoi(cmdbuff);
                          if (a==-99)
                          {
                             cprintf("\r\nARE YOU SURE YOU WANT TO QUIT? ");
                             yesno();
                             if (ans=='Y')
                             {
                                gamecomp=-99;
                                break;  /* Break out of command loop */
                             }
                             else
                                continue;  /* Back to top of command loop */
                          }
                          if ((a<1)||(a>6))
                          {
                             for (i=0;i<6;i++)
                                cprintf("  %i = %s\r\n",i+1,ds[i]);
                             cprintf("  -99 OR ESC TO QUIT\r\n\n");
                             continue;  /* Back to top of command loop */
                          }
                          switch (a)
                          {
                             case 1 :  /* Warp engines */
                                while (TRUE)
                                {
                                   while (TRUE)
                                   {
                                      getcourse();
                                      if (c<9.0) break;
                                      beep();
                                   }
                                   if (c>=1.0)
                                      while (TRUE)
                                      {
                                         getwarp();
                                         if ((w<=0.0)||(w>12.0))
                                         {
                                            c=10.0;
                                            break;
                                         }
                                         if ((d[0]>0)&&(w>0.2))
                                         {
                                            i=0;
                                            cprintf("%s DAMAGED; MAX IS 0.2; ",ds[i]);
                                            showestreptime();
                                            beep();
                                         }
                                         else
                                            break;
                                         beep();
                                      }
                                   if (c<9.0) break;
                                }
                                if (c<1.0) break;  /* Abort move */
                                checkforhits();
                                if (e<=0.0)  /* Ran out of energy */
                                {
                                   gamecomp=-1;
                                   break;
                                }
                                if (rnd()<=0.25)
                                {
                                   x=floor(rnd()*6.0);
                                   if (rnd()<=0.5)
                                   {
                                      beep();
                                      d[x]+=floor(6.0-rnd()*5.0);
                                      cprintf("**SPACE STORM, %s DAMAGED**\r\n",ds[x]);
                                      i=x;
                                      showestreptime();
                                      d[x]++;
                                      delay(100);
                                      beep();
                                   }
                                   else
                                   {
                                      j=-1;
                                      for (i=x;i<6;i++)
                                         if (d[i]>0)
                                         {
                                            j=i;
                                            break;
                                         }
                                      if (j<0)
                                         for (i=0;i<x;i++)
                                            if (d[i]>0)
                                            {
                                               j=i;
                                               break;
                                            }
                                      if (j>=0)
                                      {
                                         d[j]=1;
                                         cprintf("**SPOCK USED A NEW REPAIR TECHNIQUE**\r\n");
                                      }
                                   }
                                }
                                for (i=0;i<6;i++)
                                   if (d[i]!=0)
                                   {
                                      d[i]--;
                                      if (d[i]<=0)
                                      {
                                         d[i]=0;
                                         cprintf("%s ARE FIXED!\r\n",ds[i]);
                                         beep();
                                      }
                                   }
                                n=floor(w*8.0);
                                e=e-n-n+0.5;
                                t++;
                                sect[s1][s2]=1;
                                if (t>t9)  /* Ran out of time! */
                                {
                                   gamecomp=-1;
                                   break;
                                }
                                dopath();
                                if (e<=0.0)  /* Ran out of energy */
                                {
                                   gamecomp=-1;
                                   break;
                                }
                                moved=TRUE;
                                break;

                             case 2 :  /* Short-range scan */
                                srscan();
                                break;

                             case 3 :  /* Long-range scan */
                                lrscan();
                                break;

                             case 4 :  /* Phasers */
                                phasers();
                                if (x>0.0)
                                {
                                   if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                                   checkforhits();
                                   if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                                   if (k9<1) gamecomp=1;  /* All Klingons destroyed! */
                                   if (!gamecomp) checkcond();
                                }
                                break;

                             case 5 :  /* Photon torpedos */
                                if (d[4]>0)  /* Torpedoes damaged */
                                {
                                   cprintf("SPACE CRUD BLOCKING TUBES.  ");
                                   i=4;
                                   showestreptime();
                                   beep();
                                   break;
                                }
                                n=15;
                                if (p<1)
                                {
                                   cprintf("NO TORPEDOES LEFT!\r\n");
                                   break;
                                }
                                c=10.0;
                                while (c>=9.0)
                                {
                                   cprintf("TORPEDO ");
                                   getcourse();
                                }
                                if (c<1.0) break;  /* Abort firing of torpedo */
                                p--;
                                cprintf("TRACK: ");
                                dopath();
                                if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                                checkforhits();
                                if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                                if (k9<1) gamecomp=1;  /* All Klingons destroyed! */
                                if (!gamecomp) checkcond();
                                break;

                             case 6 :  /* Galactic records */
                                galrecs();
                                break;
                          }
                          if (gamecomp) break;
                          if (moved) break;  /* Enterprise moved */
                       }  /* End command loop */
                    }
                 }  /* Game is over! */
            */
            self.showstardate(sout)?;
            match gamecomp {
                GameState::Won => {
                    let t = self.t;
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
                    // break;
                }
                GameState::Lost => {
                    if self.t > self.game_defs.t9 {
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
                        self.k9
                    )?;
                    writeln!(sout, "YOU ARE DEMOTED TO CABIN BOY!")?;
                    // break;
                }
                GameState::Quit => {
                    writeln!(sout, "OKAY, QUITTER -- NO KUDOS FOR YOU.")?;
                    // break;
                }
                GameState::InProgress => {
                    return Err(StarTrustError::GameStateError(String::from(
                        "`gamecomp` is `InProgress`, but in game complete",
                    )))
                }
            }
        }

        Ok(())
    }
}
