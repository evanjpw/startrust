// #[macro_use]
extern crate dimensioned as dim;
#[macro_use]
extern crate derive_builder;

pub use error::{StarTrustError, StResult};
pub use stinstr::{showinst, title};

mod error;
mod the_game;
pub use the_game::{TheGame, TheGameDefs, TheGameDefsBuilder};
mod interaction;
pub use interaction::{clrscr, yesno};
mod stinstr;
mod util;
