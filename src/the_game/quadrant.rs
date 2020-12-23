use std::ops::{Index, IndexMut};

use crate::the_game::{Sector, SectorContents};
use crate::util::{findslot, setrndxy};
use crate::TheGame;

#[derive(Copy, Clone, Debug)]
pub struct Quadrant(u8, u8);

// TODO: Maybe allow invalid quadrants?
impl Quadrant {
    pub(crate) fn new(x: u8, y: u8) -> Self {
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

    pub(crate) fn is_in_range(&self) -> bool {
        // Original definition: `(q1<0)||(q1>7)||(q2<0)||(q2>7)`
        // This quadrant can never be out of range, so it's always true
        true
    }

    pub(crate) fn x(&self) -> u8 {
        self.0
    }

    pub(crate) fn y(&self) -> u8 {
        self.1
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
    pub(crate) fn new() -> Self {
        Self {
            quad: vec![vec![0i16; 8]; 8],
        }
    }
}

/// Setup a quadrant as the ship arrives
pub fn setupquad(the_game: &mut TheGame) {
    let quadrant = the_game.current_quadrant();
    let s9 = the_game.s9();
    // Set the  global "command" to "None".
    the_game.saved_command = 0.into();
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
    the_game.s = s as i32;
} /* End setupquad */
