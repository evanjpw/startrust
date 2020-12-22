//! # startrust::the_game

use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(i32)]
pub enum Command {
    #[num_enum(default)]
    Undefined = 0,
    WarpEngines = 1,
    ShortRangeScan = 2,
    LongRangeScan = 3,
    Phasers = 4,
    PhotonTorpedos = 5,
    Galacticrecords = 6,
    Quit = -99,
}
