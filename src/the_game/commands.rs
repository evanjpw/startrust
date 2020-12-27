//! # startrust::the_game

use std::convert::{Infallible, TryFrom};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use log::debug;
use num_enum::{FromPrimitive, IntoPrimitive};
use strum_macros::AsRefStr;

#[derive(AsRefStr, Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(i32)]
pub enum Command {
    #[num_enum(default)]
    #[strum(serialize = "UNDEFINED")]
    Undefined = 0,
    #[strum(serialize = "WARP ENGINES")]
    WarpEngines = 1,
    #[strum(serialize = "SHORT RANGE SENSORS")]
    ShortRangeScan = 2,
    #[strum(serialize = "LONG RANGE SENSORS")]
    LongRangeScan = 3,
    #[strum(serialize = "PHASERS")]
    Phasers = 4,
    #[strum(serialize = "PHOTON TORPEDOES")]
    PhotonTorpedos = 5,
    #[strum(serialize = "GALACTIC RECORDS")]
    GalacticRecords = 6,
    #[strum(serialize = "QUIT")]
    Quit = -99,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
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
