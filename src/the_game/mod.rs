//! # startrust::the_game

use std::io::{BufRead, Write};

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::error::StarTrustError::GameStateError;
pub use crate::the_game::config::{TheGameDefs, TheGameDefsBuilder};
use crate::the_game::quadrant::{Quadrant, QuadrantMap};
pub use crate::the_game::sector::{Sector, SectorContents, SectorMap};
use crate::the_game::stardate::StarDate;
use crate::util::{findslot, gt, lt, randinit, rnd, setrndxy};
use crate::{StResult, StarTrustError};

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
        }
    }
    /*        */
    /*
    int ,a,
        x7,y7,i,j,s,;
    double x3,y3,n,rn,h,;
    char ans,fbuff[81],cond[7],es[16],cmdbuff[8];
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

    // Display current star date
    pub fn showstardate<W: Write>(&self, sout: &mut W) -> StResult<()> {
        write!(sout, "\r\nIT IS STARDATE {}.\r\n", self.t).map_err(|e| {
            let e: StarTrustError = e.into();
            e
        })
    } /* End showstardate */

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
                    break;
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
                    break;
                }
                GameState::Quit => {
                    writeln!(sout, "OKAY, QUITTER -- NO KUDOS FOR YOU.")?;
                    break;
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
