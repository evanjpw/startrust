use std::ops::{Index, IndexMut};

use num_enum::{FromPrimitive, IntoPrimitive};

// use crate::util::{find_slot, set_random_x_y};
#[allow(unused_imports)]
use crate::TheGame;

// This has to be a byte string not a `str` because Rust worries about UTF-8 (very reasonably)
const QS: &[u8] = b"U.EKB*";

///
#[derive(Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(i32)]
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
        let index: i32 = (*self).into();
        QS[index as usize] as char
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sector(i32, i32);

impl Sector {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        if x > 7 || y > 7 {
            panic!("Could not create sector ({}, {}), value out of range", x, y)
        }
        Self(x, y)
    }

    fn values(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub(crate) fn x(&self) -> i32 {
        self.0
    }

    pub(crate) fn y(&self) -> i32 {
        self.1
    }
}

pub struct SectorMap {
    sect: Vec<Vec<i32>>,
}

impl Index<Sector> for SectorMap {
    type Output = i32;

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
            sect: vec![vec![0i32; 8]; 8],
        }
    }

    pub(crate) fn sector_contents_at(&self, sector: Sector) -> SectorContents {
        self[sector].into()
    }

    pub(crate) fn sector_contents_at_coords(&self, x: i32, y: i32) -> SectorContents {
        let sector = Sector::new(x, y);
        self.sector_contents_at(sector)
    }

    pub(crate) fn sector_char_at_coords(&self, x: i32, y: i32) -> char {
        let index = self.sector_contents_at_coords(x, y);
        index.to_char()
    }
}
