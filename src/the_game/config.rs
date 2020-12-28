use crate::the_game::stardate::StarDate;

#[derive(Builder, Copy, Clone, Debug)]
#[builder(default)]
pub struct TheGameDefs {
    /// Initial Energy
    pub(crate) initial_energy: f64, // Probably could be `f32`
    /// Initial Photon Torpedoes
    pub(crate) initial_photon_torpedoes: i32, // Probably should be `u8`
    /// Initial StarDate
    pub(crate) beginning_stardate: StarDate,
    /// Final StarDate
    pub(crate) ending_stardate: StarDate,
    /// Initial total Klingons
    pub(crate) initial_total_klingons: i32,
    pub(crate) starbase_frequency: f64,
    pub(crate) s9: f64,
    x: i32,
    y: i32,
    pub(crate) x1: f64,
    pub(crate) y1: f64,
    pub(crate) x2: f64,
    pub(crate) y2: f64,
}

impl TheGameDefs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TheGameDefs {
    fn default() -> Self {
        let e0 = 4000.0;
        let p0 = 10;
        let t0 = StarDate::new(3421);
        let t9 = StarDate::new(3451);
        let x = 8;
        let y = 1;
        let x1 = 0.2075;
        let y1 = 6.28;
        let x2 = 3.28;
        let y2 = 1.8;
        let aa = 0.96;
        let k9 = 0;
        let s9 = 400.0;

        Self {
            beginning_stardate: t0,
            ending_stardate: t9,
            initial_energy: e0,
            initial_photon_torpedoes: p0,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
            starbase_frequency: aa,
            s9,
            initial_total_klingons: k9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_builder() -> Result<(), String> {
        let the_game_defs = TheGameDefsBuilder::default().build()?;
        assert_eq!(StarDate::new(3421), the_game_defs.beginning_stardate);
        // assert_eq!(StarDate::new(3421), the_game.t);
        assert_eq!(StarDate::new(3451), the_game_defs.ending_stardate);
        Ok(())
    }
}
