use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use log::debug;
use termcolor::{Color, ColorSpec, WriteColor};

use crate::interaction::draw_number_in_color;
use crate::the_game::{find_slot, Sector, SectorContents};
use crate::util::get_random_x_y;
use crate::{StResult, TheGame};

#[derive(Copy, Clone, Debug)]
pub struct QuadrantContents {
    klingons: i32,
    pub(crate) starbases: i32,
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn decrement_klingons(&mut self) {
        self.klingons -= 1;
        assert!(self.klingons >= 0)
    }

    fn validate(&self) {
        assert!(self.klingons >= 0, "klingons: {} < 0", self.klingons);
        assert!(self.klingons < 10, "kilngons: {} >= 10", self.klingons);
        assert!(self.starbases >= 0, "starbases: {} < 0", self.starbases);
        assert!(self.starbases <= 1, "starbases: {} > 1", self.starbases);
        assert!(self.stars >= 0, "stars: {} < 0", self.stars);
        assert!(self.stars < 10, "stars: {} >= 10", self.stars);
    }

    pub fn draw<W: WriteColor>(&self, sout: &mut W, bold: bool) -> StResult<()> {
        if !self.hidden {
            draw_number_in_color(sout, self.klingons, Color::Magenta, bold)?;
            draw_number_in_color(sout, self.starbases, Color::Cyan, bold)?;
            draw_number_in_color(sout, self.stars, Color::Yellow, bold)?;
            sout.flush()?;
        } else {
            sout.set_color(ColorSpec::new().set_dimmed(true))?;
            write!(sout, "***")?;
            sout.flush()?;
            sout.reset()?;
        }
        Ok(())
    }
}

impl Default for QuadrantContents {
    fn default() -> Self {
        Self::new(0, 0, 0, true)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Quadrant(i32, i32, bool);

// TODO: Maybe allow invalid quadrants?
impl Quadrant {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        if x > 7 || y > 7 {
            debug!(
                "Could not create quadrant ({}, {}), value out of range",
                x, y
            );
            Self(0, 0, false)
        } else {
            Self(x, y, true)
        }
    }

    fn values(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub(crate) fn is_in_range(&self) -> bool {
        // Original definition: `(q1<0)||(q1>7)||(q2<0)||(q2>7)`
        // This quadrant can never be out of range, so it's always true
        self.2
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

    pub fn show_quadrant(&mut self, quadrant: Quadrant) {
        self[quadrant].show();
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
pub fn setup_quadrant(the_game: &mut TheGame) {
    let quadrant = the_game.current_quadrant();
    let s9 = the_game.s9();
    // Set the  global "command" to "None".
    the_game.saved_command = 0.into();

    let stars: i32;
    let klingons: i32;
    let starbases: i32;

    if !quadrant.is_in_range() {
        stars = 0;
        klingons = 0;
        starbases = 0;
    } else {
        let quad = &mut the_game.quadrant_map;
        let n = quad[quadrant];
        quad[quadrant].validate();
        debug!("validating quadrant {}", quadrant);
        quad.show_quadrant(quadrant);
        stars = n.stars;
        klingons = n.klingons;
        starbases = n.starbases;
    }

    let (x, y) = get_random_x_y();
    let current_sector = Sector::new(x, y);
    the_game.set_current_sector(current_sector);
    let sect = &mut the_game.sector_map;

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
        if (i as i32) < klingons {
            let sector = find_slot(sect);
            kx = sector.x();
            ky = sector.y();
            sect[sector] = SectorContents::Klingon.into();
            the_game.k3[i] = s9;
        }
        the_game.k1[i] = kx;
        the_game.k2[i] = ky;
    }
    if starbases > 0 {
        let sector = find_slot(sect);
        sect[sector] = SectorContents::Starbase.into();
    }

    for _ in 0..stars {
        let sector = find_slot(sect);
        sect[sector] = SectorContents::Star.into();
    }
    the_game.quadrant_klingons = klingons;
    the_game.quadrant_starbases = starbases;
    the_game.quadrant_stars = stars;
} /* End setupquad */
