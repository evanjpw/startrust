//!

use std::fmt::{Display, Formatter};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{AsRefStr, EnumString};
use termcolor::WriteColor;

use crate::interaction::beep;
use crate::StResult;

#[derive(AsRefStr, Debug, PartialEq, EnumString, IntoPrimitive, TryFromPrimitive, Copy, Clone)]
#[repr(usize)]
pub(crate) enum Component {
    #[strum(serialize = "WARP ENGINES")]
    WarpEngines = 0,
    #[strum(serialize = "SHORT RANGE SENSORS")]
    ShortRangeSensors = 1,
    #[strum(serialize = "LONG RANGE SENSORS")]
    LongRangeSensors = 2,
    #[strum(serialize = "PHASERS")]
    Phasers = 3,
    #[strum(serialize = "PHOTON TORPEDOES")]
    PhotonTorpedoes = 4,
    #[strum(serialize = "GALACTIC RECORDS")]
    GalacticRecords = 5,
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub(crate) struct Damage(Vec<i32>);

impl Damage {
    pub(crate) fn new() -> Self {
        Self(vec![0i32; 6])
    }

    /// Repair anything that is down
    pub fn fix_damage(&mut self) {
        for i in 0..6 {
            self.0[i] = 0;
        }
    } /* End fixdamage */

    #[allow(dead_code)]
    fn get_damage(&self, i: Component) -> i32 {
        let i: usize = i.into();
        self.0[i]
    }

    pub(crate) fn set_damage(&mut self, i: usize, value: i32) {
        self.0[i] = value
    }

    pub(crate) fn add_damage(&mut self, x: usize, increment: i32) {
        self.0[x] += increment;
    }

    pub(crate) fn is_damaged(&self, i: usize, no_negativity: bool) -> bool {
        if no_negativity {
            self.0[i] != 0
        } else {
            self.0[i] > 0
        }
    }

    pub(crate) fn reduce_and_normalize_damage(&mut self, i: usize) -> bool {
        self.0[i] -= 1;
        if self.0[i] <= 0 {
            self.0[i] = 0;
            true
        } else {
            false
        }
    }

    /// Show estimated time for repair
    pub(crate) fn show_est_repair_time<W: WriteColor>(
        &self,
        sout: &mut W,
        i: usize,
    ) -> StResult<()> {
        writeln!(sout, "{} YEARS ESTIMATED FOR REPAIR.\n", self.0[i]).map_err(|e| {
            let e = e.into();
            e
        })
    } /* End showestreptime */

    /// Show damaged item
    pub(crate) fn show_damage<W: WriteColor>(&self, sout: &mut W, i: Component) -> StResult<()> {
        write!(sout, "{} DAMAGED.  ", i.as_ref())?;
        sout.flush()?;
        beep();
        self.show_est_repair_time(sout, i.into())
    } /* End showdamage */
}
