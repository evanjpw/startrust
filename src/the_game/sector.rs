use std::ops::{Index, IndexMut};

use num_enum::{FromPrimitive, IntoPrimitive};
use crate::util::{findslot, setrndxy};
use crate::TheGame;

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

    pub(crate) fn x(&self) -> u8 {
        self.0
    }

    pub(crate) fn y(&self) -> u8 {
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
    pub(crate) fn new() -> Self {
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
        the_game.b = b as u32;
    } /* End setupquad */
}
