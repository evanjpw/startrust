use crate::error::StarTrustError;
#[allow(unused_imports)]
use crate::{StResult, TheGame};
#[allow(unused_imports)]
use beep::beep as sound;
use dim::{dimensions::Frequency, si::Hertz};
#[allow(unused_imports)]
use log::{debug, info};
use num_enum::{FromPrimitive, IntoPrimitive};
use num_traits::{Num, ToPrimitive};
use std::io::{BufRead, Read};
use std::thread;
use std::time::Duration;
use termcolor::{Color, ColorSpec, WriteColor};

const ESC_KEY: u8 = 27; /* 'ESC' key code */
const ENTER_KEY: u8 = 10; /* 'Enter' key code *///3
const MINUS_ENTER_KEY: u8 = ((-(ENTER_KEY as i32)) & 0xFF) as u8;
const NULL_C: u8 = '\0' as u8; /* Null character */
const F1_KEY: u8 = (-59 & 0xFF) as u8; /* 'F1' key code (following NULL) */
const BKSPC_KEY: u8 = 8; /* 'Backspace' key code */
const SPC: u8 = 32; /* Space character */
const ASCHI: u8 = 126; /* Maximum input character to allow */
const CTL_BKSPC_KEY: u8 = 127; /* 'Ctrl-backspace' key code */

/// Wait for the provided number of milliseconds
pub fn delay(ms: usize) {
    thread::sleep(Duration::from_millis(ms as u64));
}

fn getbyte<R: Read>(sin: &mut R) -> StResult<Option<u8>> {
    sin.bytes().next().transpose().map_err(|e| {
        let e: StarTrustError = e.into();
        e
    })
}

fn getch<R: Read>(sin: &mut R) -> StResult<Option<char>> {
    getbyte(sin).map(|option_b| option_b.map(|b| b as char))
}

fn is_ansi() -> bool {
    true
}

pub fn clrscr<W: WriteColor>(sout: &mut W) -> StResult<()> {
    let clear_sequence = if is_ansi() { "\x1b[2J\x1b[H" } else { "\x0c" };
    write!(sout, "{}", clear_sequence)?;
    sout.flush().map_err(|e| e.into())
}

fn nosound() {
    // sound(Hertz::new(0.0f64));
}

// ********************************************************************
/// Function:     speaker
/// Argument(s):  frequency of sound; duration of sound (ms)
/// Description:  Sounds the speaker with the specified frequency for the
/// specified duration in milliseconds.
/// Includes:     dos.h
///
fn speaker<HZ, N>(_freq: HZ, dur: Duration)
where
    HZ: Frequency + Into<Hertz<N>>,
    N: Num + ToPrimitive,
{
    //sound(_freq);
    thread::sleep(dur);
    nosound();
} // End speaker

// ********************************************************************
/// Function:     clearkeyboard
/// Argument(s):  none
/// Description:  Clears the keyboard buffer.
/// Includes:     conio.h
#[allow(dead_code)]
fn clearkeyboard<R: BufRead>(_stdin: &mut R) -> StResult<()> {
    // was: `while (kbhit())`
    loop {
        let c = getch(_stdin)?;
        if c.is_none() {
            return Ok(());
        }
    }
} // End clearkeyboard

// ********************************************************************
/// Function:     fgetline
/// Argument(s):  buffer, max buffer length, stream pointer
/// Description:  Gets a line of text from the stream and strips the
/// non-printing characters at the end of the line
/// Includes:     stdio.h
#[allow(dead_code)]
pub fn fgetline<R: BufRead>(_stream: &mut R) -> Result<String, StarTrustError> {
    let mut buff = String::new();
    let j = _stream.read_line(&mut buff)?;
    Ok(if j > 0 {
        let i = buff.trim_end();
        i.to_string()
    } else {
        buff
    })
} // End fgetline
  // int
  //
  // for 0;j<blen;j++ []0;
  // fgets(,blen,stream);
  // for (j=strlen()-1;j>=0;j--)
  // {
  // if (buff[j]>31) break;
  // [j]=0;
  // }_tdtodo!()
  //char *buff,int blen,FILE *stream

/// Get Y or N from user and place result in ans
pub fn yesno<R: BufRead>(
    sin: &mut R, //, W: WriteColor stdout: &mut W
) -> Result<char, StarTrustError> {
    loop {
        if let Some(c) = getch(sin)? {
            let c = c.to_uppercase().next().unwrap_or('\0');
            if "YN".contains(c) {
                return Ok(c);
            }
        } else {
            debug!("EOF in yesno")
        }
    }
} /* End yesno */

/// Get keypress to continue
#[allow(dead_code)]
pub fn keytocont<R: BufRead, W: WriteColor>(sin: &mut R, sout: &mut W) -> StResult<()> {
    write!(sout, "\nPRESS A KEY TO CONTINUE ... ")?;
    sout.flush()?;
    clearkeyboard(sin)?;
    let _ = getch(sin)?;
    clearkeyboard(sin)?;
    writeln!(sout)?;
    Ok(())
}

// ********************************************************************
/// Function:     beep
/// Argument(s):  <none>
/// Description:  Sounds the speaker with the specified frequency for the
/// specified duration in milliseconds.  Calls speaker.
pub fn beep() {
    speaker(Hertz::new(880.0f64), Duration::from_millis(80));
} // End beep

// ********************************************************************
/// Function:     buzz
/// Argument(s):  <none>
/// Description:  Sounds the speaker with the specified frequency for the
/// specified duration in milliseconds.  Calls speaker.
pub fn buzz() {
    speaker(Hertz::new(50.0f64), Duration::from_millis(200));
} // End buzz

// intcc=;cc>;cc--%c%c+  cc;int*()cprintf/, char *buff
// buff[*bl]=NULLC;
// mut case:break;default :;{}()//0pio;swi()swi()case:break;l>0%c%cl--;
//[]NULLCdefault :cprintf()int charint
// l++// buff[l]=NULLC//break;[l]=cprintf%ccase :break;case : break;case:break;
/*
int i,           return ok;

   i=;
   if ((i<03)) i=1;
   swi(i)
   {
      case 0 :                 break;
      case 1 :                 break;
      case 2 :
         break;
      case 3 :
         break;ok=ok=ok=    okok=;;;;    ok: bool;
 */

/// Check for valid input characters
fn charokay(cc: u8, mode: InputMode) -> bool {
    match mode {
        InputMode::Mode0 => (cc >= (' ' as u8)) && (cc <= ASCHI),
        InputMode::Mode1 => {
            (cc >= ('A' as u8)) && (cc <= ('Z' as u8)) || (cc == ('*' as u8)) || (cc == (' ' as u8))
        }
        InputMode::Mode2 => {
            ((cc >= ('0' as u8)) && (cc <= ('9' as u8)))
                || (cc == ('.' as u8))
                || (cc == (',' as u8))
                || (cc == ('-' as u8))
        }
        InputMode::Mode3 => {
            ((cc >= ('A' as u8)) && (cc <= ('Z' as u8)))
                || ((cc >= ('0' as u8)) && (cc <= ('9' as u8)))
                || (cc == (' ' as u8))
        }
        InputMode::InvalidMode => false,
    }
} /* End charokay */

/// Do a Ctrl-Backspace
fn ctlbkspc<W: WriteColor>(sout: &mut W, bl: &mut i32) -> StResult<()> {
    if *bl > 0 {
        for _ in 0..(*bl) {
            write!(sout, "{} {}", BKSPC_KEY as char, BKSPC_KEY as char)?;
            sout.flush()?;
        }
        *bl = 0;
    }
    Ok(())
} /* End ctlbkspc */

#[derive(Copy, Clone, Debug, IntoPrimitive, FromPrimitive, Eq, PartialEq)]
#[repr(i32)]
pub enum InputMode {
    /// all alphanumeric characters allowed
    Mode0 = 0,
    /// alphabetic, space, asterisk, function keys
    Mode1 = 1,
    /// digits, '-', ',', '.' only
    Mode2 = 2,
    /// alphabetic, space, and digits only
    Mode3 = 3,
    #[num_enum(default)]
    ///
    InvalidMode = -1,
}

pub enum InputValue {
    InputString(String),
    Esc,
    Blank,
}

impl InputValue {
    #[allow(dead_code)]
    fn as_integer(&self) -> i32 {
        match self {
            InputValue::InputString(_) => 0,
            InputValue::Esc => -1,
            InputValue::Blank => 1,
        }
    }
}

/**  ********************************************************************
    Function:     getinp
    Argument(s):  input buffer, maximum input length, mode
    Description:  Gets input from console, echoing to the current screen
                  position.  An attempt to enter more characters than the
                  maximum length will result in a beep at the terminal.
                  Backspace erases the most recent character.  Ctrl-
                  backspace erases the entire line.  An attempt to erase
                  non-existent characters will result in a buzz at the
                  terminal.  The mode determines what input will be
                  accepted:
                     mode = 0: all alphanumeric characters allowed
                     mode = 1: alphabetic, space, asterisk, function keys
                     mode = 2: digits, '-', ',', '.' only
                     mode = 3: alphabetic, space, and digits only
                  Note that for this program, all alphabetic charaters are
                  shifted to upper case.

                  If a mode other than 0, 1, 2 or 3 is passed, the default
                  mode is mode 0.  Function key F1 is treated like a CR only
                  in mode 2.
                  Other keys, like Alt- keys, etc., result in a beep at the
                  terminal.  Calls beep and buzz.
    Returns:      0 for successful read; 1 for CR only; -1 for ESC
    Includes:     conio.h
*/
pub fn getinp<R: BufRead, W: WriteColor>(
    sin: &mut R,
    sout: &mut W,
    _length: usize,
    mode: InputMode,
) -> StResult<InputValue> {
    let md: i32 = mode.into();
    let mut cc = 0;
    let mut buff: Vec<u8> = Vec::new();
    while (cc != ENTER_KEY) && (cc != ESC_KEY) && (cc != MINUS_ENTER_KEY) {
        debug!("cc = {} ({})", cc, cc as char);
        if let Some(the_char) = getch(sin)? {
            cc = the_char as u8;
        } else {
            continue;
        }
        if cc == NULL_C
        /*  Function key hit.  */
        {
            if let Some(n_cc) = getch(sin)? {
                let n_cc = (-(n_cc as i32) & 0xFF) as u8;
                let mut l = buff.len() as i32;
                if md == 2.into() {
                    match n_cc {
                        F1_KEY => {
                            /*  F1;  Help - same as blank CR  */
                            if l > 0 {
                                ctlbkspc(sout, &mut l)?;
                                buff.clear()
                            }
                            cc = ENTER_KEY;
                        }
                        _ => buzz(),
                    }
                } else {
                    buzz();
                }
            } else {
                buzz();
            }
        } else if (cc < BKSPC_KEY)
            || ((cc > BKSPC_KEY) && (cc < ENTER_KEY))
            || ((cc > ENTER_KEY) && (cc < ESC_KEY))
            || ((cc > ESC_KEY) && (cc < SPC))
            || (cc > ASCHI)
        {
            beep();
        } else {
            match cc {
                BKSPC_KEY => {
                    /*  Perform destructive backspace.  */
                    if !buff.is_empty() {
                        write!(sout, "{} {}", cc as char, cc as char)?;
                        sout.flush()?;
                        let _ = buff.pop();
                    } else {
                        buzz();
                    }
                }
                ESC_KEY => {
                    /*  Perform escape (abort input).  */
                    let mut l = buff.len() as i32;
                    if l > 0 {
                        ctlbkspc(sout, &mut l)?;
                    }
                }
                CTL_BKSPC_KEY => {
                    /*  Perform Ctrl-backspace.  */
                    let mut l = buff.len() as i32;
                    if l > 0 {
                        ctlbkspc(sout, &mut l)?;
                    } else {
                        buzz();
                    }
                }
                ENTER_KEY => { /*  Carriage return -- don't do anything  */ }
                _ => {
                    // Possibly valid ASCII character
                    if buff.len() < _length {
                        cc = ((cc as char).to_uppercase().next().ok_or(
                            StarTrustError::GeneralError(format!(
                                "Error converting {} to upper case",
                                cc
                            )),
                        )?) as u8;
                        if charokay(cc, md.into()) {
                            buff.push(cc);
                            write!(sout, "{}", cc as char)?;
                            sout.flush()?;
                        } else {
                            beep();
                        }
                    } else {
                        beep();
                    }
                }
            }
        }
    }
    Ok(if cc == ESC_KEY {
        // return -1;
        InputValue::Esc
    } else if buff.is_empty() {
        // return 1;
        InputValue::Blank
    } else {
        // return 0;
        InputValue::InputString(String::from_utf8(buff).unwrap())
    })
} /* End getinp */

/// Gets course and places in variable c
pub fn getcourse<R: BufRead, W: WriteColor>(
    sin: &mut R,
    sout: &mut W,
    // the_game: &TheGame,
) -> StResult<f64> {
    write!(sout, "COURSE (1-8.99)? ")?;
    sout.flush()?;

    let gb = getinp(sin, sout, 4, 2.into());

    writeln!(sout)?;

    Ok(if let InputValue::InputString(ibuff) = gb? {
        ibuff.parse()?
    } else {
        0.0
    })
} /* End getcourse */

pub fn getwarp<R: BufRead, W: WriteColor>(sin: &mut R, sout: &mut W) -> StResult<f64> {
    // Gets warp and places in variable w
    write!(sout, "WARP (0-12.0)? ")?;
    sout.flush()?;
    let gb = getinp(sin, sout, 4, 2.into())?;
    writeln!(sout)?;
    Ok(if let InputValue::InputString(ibuff) = gb {
        ibuff.parse()?
    } else {
        0.0
    })
} /* End getwarp */

/// Draw one number in one color
pub fn draw_number_in_color<W: WriteColor>(
    sout: &mut W,
    e: i32,
    color: Color,
    bold: bool,
) -> StResult<()> {
    let mut color_spec = ColorSpec::new();
    sout.set_color(
        color_spec
            .set_fg(if e != 0 { Some(color) } else { None })
            .set_bold(bold)
            .set_intense(bold),
    )?;
    write!(sout, "{}", e)?;
    sout.reset()?;
    Ok(())
}
