//! # startrust::the_game

use std::fmt::{Display, Formatter};
use std::io::Write;
use std::ops;
use std::ops::{Index, IndexMut};

use num_enum::{FromPrimitive, IntoPrimitive};
// use num_traits::abs;

use crate::util::{findslot, setrndxy};
use crate::{StResult, StarTrustError};


//O
const DS: &'static [&'static str] = &[
    "WARP ENGINES",
    "SHORT RANGE SENSORS",
    "LONG RANGE SENSORS",
    "PHASERS",
    "PHOTON TORPEDOES",
    "GALACTIC RECORDS",
];

#[derive(Copy, Clone, Debug)]
pub struct Quadrant(u8, u8);

// TODO: Maybe allow invalid quadrants?
impl Quadrant {
    fn new(x: u8, y: u8) -> Self {
        if x > 7 || y > 7 {
            panic!(
                "Could not create quadrant ({}, {}), value out of range",
                x, y
            )
        }
        Self(x, y)
    }

    fn values(&self) -> (u8, u8) {
        (self.0, self.1)
    }

    fn is_in_range(&self) -> bool {
        // Original definition: `(q1<0)||(q1>7)||(q2<0)||(q2>7)`
        // This quadrant can never be out of range, so it's always true
        true
    }
}

impl Index<Quadrant> for QuadrantMap {
    type Output = i16;

    fn index(&self, index: Quadrant) -> &Self::Output {
        let (q1, q2) = index.values();
        &self.quad[q1 as usize][q2 as usize]
    }
}

impl IndexMut<Quadrant> for QuadrantMap {
    fn index_mut(&mut self, index: Quadrant) -> &mut Self::Output {
        let (q1, q2) = index.values();
        &mut self.quad[q1 as usize][q2 as usize]
    }
}

pub struct QuadrantMap {
    quad: Vec<Vec<i16>>,
}

impl QuadrantMap {
    fn new() -> Self {
        Self {
            quad: vec![vec![0i16; 8]; 8],
        }
    }
}

// This has to be a byte string not a `str` because Rust worries about UTF-8 (very reasonably)
const QS: &[u8] = b"U.EKB*";

///
#[derive(Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum SectorContents {
    #[num_enum(default)]
    Unknown = 0,
    Empty = 1,
    Enterprise = 2,
    Klingon = 3,
    Starbase = 4,
    Star = 5,
}

impl SectorContents {
    pub fn to_char(&self) -> char {
        let index: u8 = (*self).into();
        QS[index as usize] as char
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sector(u8, u8);

impl Sector {
    pub(crate) fn new(x: u8, y: u8) -> Self {
        if x > 7 || y > 7 {
            panic!("Could not create sector ({}, {}), value out of range", x, y)
        }
        Self(x, y)
    }

    fn values(&self) -> (u8, u8) {
        (self.0, self.1)
    }

    fn x(&self) -> u8 {
        self.0
    }

    fn y(&self) -> u8 {
        self.1
    }
}

pub struct SectorMap {
    sect: Vec<Vec<u8>>,
}

impl Index<Sector> for SectorMap {
    type Output = u8;

    fn index(&self, index: Sector) -> &Self::Output {
        let (x, y) = index.values();
        &self.sect[x as usize][y as usize]
    }
}

impl IndexMut<Sector> for SectorMap {
    fn index_mut(&mut self, index: Sector) -> &mut Self::Output {
        let (x, y) = index.values();
        &mut self.sect[x as usize][y as usize]
    }
}

impl SectorMap {
    fn new() -> Self {
        Self {
            sect: vec![vec![0u8; 8]; 8],
        }
    }

    pub(crate) fn sector_contents_at(&self, sector: Sector) -> SectorContents {
        self[sector].into()
    }

    pub(crate) fn sector_contents_at_coords(&self, x: u8, y: u8) -> SectorContents {
        let sector = Sector::new(x, y);
        self.sector_contents_at(sector)
    }

    //_from_coordsthe_game.()
    //u8()=<;i++(;j++);j<=1[](=;<;i++)   mut      /*              x[y]3*/()/*  ; ; ;  */
    // &mut s1=       s2=;sect[s1][s2]=2;()x[y];i>0)=;while (           i--;x[y];54// }i,valu
    /// Set up quadrant when entering it
    fn setupquad(the_game: &mut TheGame) {
        let quadrant = the_game.current_quadrant();
        let s9 = the_game.s9();
        // TODO: I recall needing `a`, but it seems like it wasn't used. Maybe it is to set the
        //  global "command" to "None".
        // let mut a = 0;
        let n: usize;
        let s: usize;
        let k: usize;

        if !quadrant.is_in_range() {
            n = 0;
            s = 0;
            k = 0;
        } else {
            let quad = &mut the_game.quad;
            n = quad[quadrant].abs() as usize;
            quad[quadrant] = n as i16;
            s = n - (n / 10) * 10;
            k = n / 100;
        }
        let b: usize = (n as f64 / 10.0f64 - (k * 10) as f64).floor() as usize;
        let (x, y) = setrndxy();
        let current_sector = Sector::new(x, y);
        the_game.set_current_sector(current_sector);
        let sect = &mut the_game.sect;

        for i in 0..8 {
            for j in 0..8 {
                sect[Sector::new(i, j)] = SectorContents::Empty.into();
            }
        }

        sect[current_sector] = SectorContents::Enterprise.into();

        let mut ky = y;
        let mut kx: u8;
        for i in 0..8 {
            the_game.k3[i] = 0.0;
            kx = 8;
            if i < k {
                let sector = findslot(sect);
                kx = sector.x();
                ky = sector.y();
                sect[sector] = SectorContents::Klingon.into();
                the_game.k3[i] = s9;
            }
            the_game.k1[i] = kx;
            the_game.k2[i] = ky;
        }
        if b > 0 {
            let sector = findslot(sect);
            sect[sector] = SectorContents::Starbase.into();
        }

        for _ in 0..s {
            let sector = findslot(sect);
            sect[sector] = SectorContents::Star.into();
        }
    } /* End setupquad */
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Default)]
pub struct StarDate(u16);

impl StarDate {
    fn new(t: u16) -> Self {
        Self(t)
    }
}

impl Display for StarDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<I: Into<u16>> ops::Add<I> for StarDate {
    type Output = StarDate;

    fn add(self, rhs: I) -> StarDate {
        StarDate(self.0 + rhs.into())
    }
}

impl<I: Into<u16>> ops::AddAssign<I> for StarDate {
    fn add_assign(&mut self, rhs: I) {
        *self = StarDate(self.0 + rhs.into());
    }
}

impl ops::Sub<StarDate> for StarDate {
    type Output = u16;

    fn sub(self, rhs: StarDate) -> u16 {
        self.0 - rhs.0
    }
}

#[derive(Builder, Copy, Clone, Debug)]
#[builder(default)]
pub struct TheGameDefs {
    /// Initial Energy
    e0: f64, // Probably could be `f32`
    /// Initial Photon Torpedoes
    p0: u16, // Probably should be `u8`
    /// Initial StarDate
    t0: StarDate,
    /// Final StarDate
    t9: StarDate,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x: u16,
    y: u16,
    aa: f64,
    w: f64,
    c: f64,
    s9: f64,
    k9: u16,
    b9: u16,
    // #[builder(setter(skip))] #[builder]
}

impl TheGameDefs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TheGameDefs {
    fn default() -> Self {
        let e0 = 4000.0;
        let p0 = 10;
        let t0 = StarDate::new(3421);
        let t9 = StarDate::new(3451);
        let x = 8;
        let y = 1;
        let x1 = 0.2075;
        let y1 = 6.28;
        let x2 = 3.28;
        let y2 = 1.8;
        let aa = 0.96;
        let c = 100 as f64;
        let w = 10 as f64;
        let k9 = 0;
        let b9 = 0;
        let s9 = 400.0;
        Self {
            t0,
            t9,
            e0,
            p0,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
            aa,
            c,
            w,
            s9,
            k9,
            b9,
        }
    }
}

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
    k3: Vec<f64>,
    k1: Vec<u8>,
    k2: Vec<u8>,
    /// The Sector Map
    pub(crate) sect: SectorMap,
    /// The Quadrant Map
    pub(crate) quad: QuadrantMap,
    pub(crate) game_defs: TheGameDefs,
}

/*
int d[6],a,newquad,
    x7,y7,k,i,j,b,k0,s,;
double x3,y3,n,rn,h,;
char ans,fbuff[81],cond[7],es[16],cmdbuff[8];
 *///

#[derive(Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(i32)]
enum GameState {
    #[num_enum(default)]
    InProgress = 0,
    Won = 1,
    Lost = -1,
    Quit = -99,
}

impl TheGame {
    pub fn new(the_game_defs: &TheGameDefs) -> Self {
        // s1 & s2 and q1 & q2 are not set initially, we will use (0, 0) the game
        // initialization will randomize them later
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
            k3: vec![0.0; 8],
            k1: vec![0u8; 8],
            k2: vec![0u8; 8],
            game_defs: *the_game_defs,
        }
    }
    pub fn increment_year(&mut self) {
        self.years -= 1;
        self.t += 1u16;
    }

    fn current_sector(&self) -> Sector {
        Sector::new(self.s1, self.s2)
    }

    fn set_current_sector(&mut self, sector: Sector) {
        self.set_current_quadrant_from_coords(sector.0, sector.1)
    }

    fn set_current_sector_from_coords(&mut self, x: u8, y: u8) {
        self.s1 = x;
        self.s2 = y;
    }

    fn current_quadrant(&self) -> Quadrant {
        Quadrant::new(self.q1, self.q2)
    }

    fn set_current_quadrant(&mut self, quadrant: Quadrant) {
        self.set_current_quadrant_from_coords(quadrant.0, quadrant.1);
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

    pub fn play(&mut self) -> StResult<()> {
        /*
    int gamecomp;
    intrating,moved;
    double drate;

          init();
          gamecomp=FALSE;
          newquad=TRUE;
          while (!gamecomp)
          {
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
          showstardate();
          switch (gamecomp)
          {
             case 1 :
                drate=t-t0;
                rating=(k0/drate)*1000.0;
                cprintf("THE FEDERATION HAS BEEN SAVED!\r\n");
                cprintf("YOU ARE PROMOTED TO ADMIRAL.\r\n");
                cprintf("%i KLINGONS IN %i YEARS.  RATING = %i\r\n\n",
                   k0,t-t0,rating);
                break;
             case -1 :
                if (t>t9)
                   cprintf("YOU RAN OUT OF TIME!\r\n");
                if (e<=0.0)
                   cprintf("YOU RAN OUT OF ENERGY!\r\n");
                cprintf("THANKS TO YOUR BUNGLING, THE FEDERATION WILL BE\r\n");
                cprintf("CONQUERED BY THE REMAINING %i KLINGON CRUISERS!\r\n",k9);
                cprintf("YOU ARE DEMOTED TO CABIN BOY!\r\n");
                break;
             case -99 :
                cprintf("OKAY, QUITTER -- NO KUDOS FOR YOU.\r\n");
                break;
          }

     */
    Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_builder() -> Result<(), String> {
        let the_game_defs = TheGameDefsBuilder::default().build()?;
        assert_eq!(StarDate(3421), the_game_defs.t0);
        // assert_eq!(StarDate(3421), the_game.t);
        assert_eq!(StarDate(3451), the_game_defs.t9);
        Ok(())
    }
}
