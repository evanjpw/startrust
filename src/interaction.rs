use crate::error::StarTrustError;
use crate::StResult;
use beep::beep as sound;
use dim::{dimensions::Frequency, si::Hertz};
use num_traits::{Num, ToPrimitive};
use std::io::{BufRead, Write};
use std::thread;
use std::time::Duration;

pub fn clrscr<W: Write>(sout: &mut W) -> StResult<()> {
    write!(sout, "\x0c").map_err(|e| e.into())
}

fn nosound() {
    sound(Hertz::new(0.0f64));
}

// ********************************************************************
/// Function:     speaker
/// Argument(s):  frequency of sound; duration of sound (ms)
/// Description:  Sounds the speaker with the specified frequency for the
/// specified duration in milliseconds.
/// Includes:     dos.h
///
fn speaker<HZ, N>(freq: HZ, dur: Duration)
where
    HZ: Frequency + Into<Hertz<N>>,
    N: Num + ToPrimitive,
{
    sound(freq);
    thread::sleep(dur);
    nosound();
} // End speaker

// ********************************************************************
/// Function:     clearkeyboard
/// Argument(s):  none
/// Description:  Clears the keyboard buffer.
/// Includes:     conio.h
fn clearkeyboard<R: BufRead>(_stdin: R) {
    // while (kbhit()) getch();
} // End clearkeyboard

// ********************************************************************
/// Function:     fgetline
/// Argument(s):  buffer, max buffer length, stream pointer
/// Description:  Gets a line of text from the stream and strips the
/// non-printing characters at the end of the line
/// Includes:     stdio.h
pub fn fgetline<R: BufRead>(_stream: R) -> Result<String, StarTrustError> //char *buff,int blen,FILE *stream
{
    // int j;
    //
    // for (j=0;j<blen;j++) buff[i]=0;
    // fgets(buff,blen,stream);
    // for (j=strlen(buff)-1;j>=0;j--)
    // {
    // if (buff[j]>31) break;
    // buff[j]=0;
    // }
    todo!()
} // End fgetline

/// Get Y or N from user and place result in ans
pub fn yesno<R: BufRead, W: Write>(_stdin: &R, stdout: &mut W) -> Result<char, StarTrustError> {
    // ans='X';
    // while ((ans!='Y')&&(ans!='N'))
    // {
    // beep();
    // clearkeyboard();
    // ans=toupper(getch());
    // }
    // cprintf("%c\r\n",ans);
    todo!()
} /* End yesno */

/// Get keypress to continue
pub fn keytocont() {
    // cprintf("\r\nPRESS A KEY TO CONTINUE ... ");
    // clearkeyboard();
    // getch();
    // clearkeyboard();
    // cprintf("\r\n");
    todo!()
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
