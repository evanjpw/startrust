//! This is an adaptation of an old text-based Star Trek game!
//! This C program is based on a BASIC program adapted by
//! L.E. Cochran 2/29/1976.  In keeping with the original
//! BASIC paradigm, almost all variables and constants are
//! global.
//!
//! To show how much can be done with a relatively small amount
//! of code, the BASIC program was about 2-1/2 pages (150 lines),
//! although almost every line included multiple statements.
//!
//! Bob Sorem -- 28 November 2000

extern crate startrust;

use std::io::{stdin, Write};

use atty;
use clap::{crate_authors, crate_description, crate_version, Clap};
use log::{debug, LevelFilter};
use termcolor::{ColorChoice, StandardStream, WriteColor};

use startrust::{
    clrscr, show_instructions, show_title, yesno, StResult, StarTrustError, TheGame, TheGameDefs,
    TheGameDefsBuilder,
};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
struct GetOpts {
    /// Don't use color in output
    #[clap(long)]
    no_color: bool,
    /// Try to force color in output
    #[clap(long)]
    force_color: bool,
    /// Run with debug output
    #[clap(short, long)]
    debug: bool,
}

fn get_game_config() -> StResult<TheGameDefs> {
    let the_game_defs = TheGameDefsBuilder::default()
        .build()
        .map_err(|e| StarTrustError::GeneralError(e))?;
    Ok(the_game_defs)
}

fn init_logger(get_opts: &GetOpts) {
    let mut builder = pretty_env_logger::formatted_builder();
    if let Ok(s) = ::std::env::var("RUST_LOG") {
        builder.parse_filters(&s);
    }
    if get_opts.debug {
        builder.filter_level(LevelFilter::Debug);
        debug!("Debug is *ON*");
    }
    builder.init();
}

fn main() -> Result<(), StarTrustError> {
    let get_opts = GetOpts::parse();
    init_logger(&get_opts);
    let sin = stdin();
    let choice = if !atty::is(atty::Stream::Stdout) || get_opts.no_color {
        ColorChoice::Never
    } else if get_opts.force_color {
        ColorChoice::Always
    } else {
        ColorChoice::Auto
    };
    let mut sout = StandardStream::stdout(choice);
    show_title(&mut sout)?;
    show_instructions(&mut sin.lock(), &mut sout)?;

    let the_game_config = get_game_config()?;

    loop {
        let mut the_game = TheGame::new(&the_game_config);

        debug!("About to print title");

        clrscr(&mut sout)?;
        show_title(&mut sout)?;

        let _game = the_game.play(&mut sin.lock(), &mut sout)?;

        let _ = write!(sout, "\nTRY AGAIN? ")?;
        sout.flush()?;
        let ans = yesno(&mut sin.lock())?;
        if ans != 'Y' {
            sout.reset()?;
            writeln!(sout)?;
            return Ok(());
        }
    }
} /* End main */
