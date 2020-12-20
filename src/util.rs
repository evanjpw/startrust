//! # startrust::util
//!
//random, , , vvoi**voidvoidwhile (TRUE)sect[][]<2break;(u8, u8)_coordsx, y
use crate::the_game::{Sector, SectorContents, SectorMap};
use rand::{thread_rng, Rng};

/// Set a random x and y in interval \[0,7\]
pub fn setrndxy() -> (u8, u8) {
    let x: u8 = thread_rng().gen_range(0..8);
    let y: u8 = thread_rng().gen_range(0..8);
    (x, y)
} /* End setrndxy */

/// Find an unoccupied sector
pub fn findslot(sector_map: &SectorMap) -> Sector {
    loop {
        let (x, y) = setrndxy();
        let sector = Sector::new(x, y);
        if sector_map.sector_contents_at(sector) == SectorContents::Empty {
            return sector;
        }
    }
} /* End findslot */
