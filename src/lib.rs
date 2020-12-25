#[macro_use]
extern crate derive_builder;

extern crate dimensioned as dim;

pub use error::{StResult, StarTrustError};
pub use interaction::{clrscr, yesno};
pub use stinstr::{show_instructions, show_title};
pub use the_game::{TheGame, TheGameDefs, TheGameDefsBuilder};

mod error;
mod interaction;
mod stinstr;
mod the_game;
mod util;
