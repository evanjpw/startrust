use std::ops::{Index, IndexMut};

use crate::the_game::{Sector, SectorContents};
use crate::util::{findslot, setrndxy};
use crate::TheGame;
use std::fmt::{Display, Formatter};
// use log::{debug};

#[derive(Copy, Clone, Debug)]
pub struct QuadrantContents {
    klingons: i32,
    starbases: i32,
    stars: i32,
    hidden: bool,
}

impl QuadrantContents {
    pub fn new(klingons: i32, starbases: i32, stars: i32, hidden: bool) -> Self {
        Self {
            klingons,
            starbases,
            stars,
            hidden,
        }
    }

    pub fn from_i32(quadrant_contents: i32) -> Self {
        let hidden = !quadrant_contents.is_positive();
        let quadrant_contents = quadrant_contents.abs();
        let stars = quadrant_contents % 10;
        let starbases = (quadrant_contents / 10) % 10;
        let klingons = quadrant_contents / 100;
        assert!(
            klingons < 10 || klingons > 0,
            "klingons({}) >= 10 or < 0, quadrant contents was {}",
            klingons,
            quadrant_contents
        );
        Self {
            klingons,
            starbases,
            stars,
            hidden,
        }
    }

    pub fn as_i32(&self) -> i32 {
        let value = self.klingons * 100 + self.starbases * 10 + self.stars;
        if self.hidden {
            -value
        } else {
            value
        }
    }

    pub fn show(&mut self) {
        self.hidden = false;
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn decrement_klingons(&mut self) {
        self.klingons -= 1;
        assert!(self.klingons >= 0)
    }
}

impl Default for QuadrantContents {
    fn default() -> Self {
        Self::new(0, 0, 0, true)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Quadrant(i32, i32);

// TODO: Maybe allow invalid quadrants?
impl Quadrant {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        if x > 7 || y > 7 {
            panic!(
                "Could not create quadrant ({}, {}), value out of range",
                x, y
            )
        }
        Self(x, y)
    }

    fn values(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub(crate) fn is_in_range(&self) -> bool {
        // Original definition: `(q1<0)||(q1>7)||(q2<0)||(q2>7)`
        // This quadrant can never be out of range, so it's always true
        true
    }

    pub(crate) fn x(&self) -> i32 {
        self.0
    }

    pub(crate) fn y(&self) -> i32 {
        self.1
    }
}

impl Display for Quadrant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quadrant({}, {})", self.x(), self.y())
    }
}

pub struct QuadrantMap {
    quad: Vec<Vec<QuadrantContents>>,
}

impl QuadrantMap {
    pub(crate) fn new() -> Self {
        Self {
            quad: vec![vec![QuadrantContents::default(); 8]; 8],
        }
    }
}

impl Index<Quadrant> for QuadrantMap {
    type Output = QuadrantContents;

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

/// Setup a quadrant as the ship arrives
pub fn setupquad(the_game: &mut TheGame) {
    let quadrant = the_game.current_quadrant();
    let s9 = the_game.s9();
    // Set the  global "command" to "None".
    the_game.saved_command = 0.into();
    let n: i32;
    let s: i32;
    let k: i32;

    if !quadrant.is_in_range() {
        n = 0;
        s = 0;
        k = 0;
    } else {
        let quad = &mut the_game.quad;
        n = quad[quadrant].as_i32().abs() as i32;
        let int_n = n as i32;
        assert!(int_n >= -999 || int_n <= 999);
        quad[quadrant] = QuadrantContents::from_i32(int_n);
        s = n - (n / 10) * 10;
        k = n / 100;
    }
    let b: i32 = (n / 10 - (k * 10));
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
    let mut kx: i32;
    for i in 0..8 {
        the_game.k3[i] = 0.0;
        kx = 8;
        if (i as i32) < k {
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
    the_game.k = k;
    the_game.b = b as i32;
    the_game.s = s as i32;
} /* End setupquad */
