use std::ops::{Index, IndexMut};

use crate::util::{findslot, setrndxy};
use crate::TheGame;
use num_enum::{FromPrimitive, IntoPrimitive};

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

    pub(crate) fn sector_char_at_coords(&self, x: u8, y: u8) -> char {
        let index = self.sector_contents_at_coords(x, y);
        index.to_char()
    }
}
