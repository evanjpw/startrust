#[macro_use]
extern crate dimensioned as dim;

pub use error::StarTrustError;
pub use stinstr::{showinst, title};

mod error;
mod the_game;
pub use the_game::TheGame;
mod interaction;
pub use interaction::{clrscr, yesno};
mod stinstr;

pub type StResult<T> = std::result::Result<T, StarTrustError>;
