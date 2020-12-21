use crate::the_game::stardate::StarDate;

#[derive(Builder, Copy, Clone, Debug)]
#[builder(default)]
pub struct TheGameDefs {
    /// Initial Energy
    pub(crate) e0: f64, // Probably could be `f32`
    /// Initial Photon Torpedoes
    pub(crate) p0: u16, // Probably should be `u8`
    /// Initial StarDate
    pub(crate) t0: StarDate,
    /// Final StarDate
    pub(crate) t9: StarDate,
    pub(crate) x1: f64,
    pub(crate) y1: f64,
    pub(crate) x2: f64,
    pub(crate) y2: f64,
    x: u16,
    y: u16,
    pub(crate) aa: f64,
    pub(crate) s9: f64,
    pub(crate) k9: u16,
    // #[builder(setter(skip))] #[builder]
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
            t0,
            t9,
            e0,
            p0,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
            aa,
            s9,
            k9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_builder() -> Result<(), String> {
        let the_game_defs = TheGameDefsBuilder::default().build()?;
        assert_eq!(StarDate::new(3421), the_game_defs.t0);
        // assert_eq!(StarDate::new(3421), the_game.t);
        assert_eq!(StarDate::new(3451), the_game_defs.t9);
        Ok(())
    }
}
