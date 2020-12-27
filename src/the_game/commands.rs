//! # startrust::the_game

use std::convert::{Infallible, TryFrom};
use std::str::FromStr;

use log::debug;
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

impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i32::from_str(s) {
            Ok(i) => Command::try_from(i),
            Err(e) => {
                debug!("Error {} converting \"{}\" => i32 => Command", e, s);
                Ok(Command::Undefined)
            }
        }
    }
}
