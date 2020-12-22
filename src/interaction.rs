use crate::error::StarTrustError;
use crate::StResult;
use beep::beep as sound;
use dim::{dimensions::Frequency, si::Hertz};
use log::debug;
use num_enum::{FromPrimitive, IntoPrimitive};
use num_traits::{Num, ToPrimitive};
use std::io::{BufRead, Read, Write};
use std::thread;
use std::time::Duration;

fn getbyte<R: Read>(sin: &mut R) -> StResult<Option<u8>> {
    sin.bytes().next().transpose().map_err(|e| {
        let e: StarTrustError = e.into();
        e
    })
}

fn getch<R: Read>(sin: &mut R) -> StResult<Option<char>> {
    getbyte(sin).map(|option_b| option_b.map(|b| b as char))
}

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
    // }_td
    todo!()
} // End fgetline

/// Get Y or N from user and place result in ans
pub fn yesno<R: BufRead, W: Write>(sin: &mut R, stdout: &mut W) -> Result<char, StarTrustError> {
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

const ESCKEY: u8 = 27; /* 'ESC' key code */
/*

#define NULLC       '\0'  /* Null character */
#define F1KEY        -59  /* 'F1' key code (following NULL) */
#define BKSPCKEY       8  /* 'Backspace' key code */
#define ENTERKEY      13  /* 'Enter' key code */
        #define
#define SPC           32  /* Space character */
#define ASCHI        126  /* Maximum input character to allow */
#define CTLBKSPCKEY  127  /* 'Ctrl-backspace' key code */

char *[6]
   {};


int charokay(char cc,int mode)  /* Check for valid input characters */
{
int i,ok;

   i=mode;
   if ((i<0)||(i>3)) i=1;
   switch(i)
   {
      case 0 :
         ok=((cc>=' ')&&(cc<=ASCHI));
         break;
      case 1 :
         ok=(((cc>='A')&&(cc<='Z'))||(cc=='*')||(cc==' '));
         break;
      case 2 :
         ok=(((cc>='0')&&(cc<='9'))||(cc=='.')||(cc==',')||(cc=='-'));
         break;
      case 3 :
         ok=(((cc>='A')&&(cc<='Z'))||((cc>='0')&&(cc<='9'))||(cc==' '));
         break;
   }
   return ok;
}  /* End charokay */


void ctlbkspc(int *bl,char *buff)  /* Do a Ctrl-Backspace */
{
int
   cc;

   if (*bl>0)
   {
      for (cc=*bl;cc>0;cc--)
         cprintf("%c %c",BKSPCKEY,BKSPCKEY);
      *bl=0;
      buff[*bl]=NULLC;
   }
}  /* End ctlbkspc */

 */

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
    fn as_integer(&self) -> i32 {
        match self {
            InputValue::InputString(_) => 0,
            InputValue::Esc => -1,
            InputValue::Blank => 1,
        }
    }
}

/*  ********************************************************************
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
pub fn getinp(_length: usize, mode: InputMode) -> InputValue {
    let md: i32 = mode.into();
    // let mut l=0;
    let mut cc = 0;
    let mut buff: Vec<u8> = Vec::new();
    /*
    while ((cc!=ENTERKEY)&&(cc!=ESCKEY)&&(cc!=-ENTERKEY))
    {
       cc=getch();
       if (cc==NULLC)   /*  Function key hit.  */
       {
          cc=-getch();
          if (md==2)
             switch (cc)
             {
                case F1KEY:  /*  F1;  Help - same as blank CR  */
                   if (l>0)
                      ctlbkspc(&l,buff);
                   cc=ENTERKEY;
                   break;
                default : buzz();
             }
          else
             buzz();
       }
       else if ((cc<BKSPCKEY)||((cc>BKSPCKEY)&&(cc<ENTERKEY))||
          ((cc>ENTERKEY)&&(cc<ESCKEY))||((cc>ESCKEY)&&(cc<SPC))||(cc>ASCHI))
          beep();
       else
          switch(cc)
          {
          case BKSPCKEY :  /*  Perform destructive backspace.  */
             if (l>0)
             {
                cprintf("%c %c",cc,cc);
                l--;
                buff[l]=NULLC;
             }
             else
                buzz();
             break;
          case ESCKEY :   /*  Perform escape (abort input).  */
             if(l>0)
                ctlbkspc(&l,buff);
             break;
          case CTLBKSPCKEY :   /*  Perform Ctrl-backspace.  */
             if (l>0)
                ctlbkspc(&l,buff);
             else
                buzz();
             break;
          case ENTERKEY :  /*  Carriage return -- don't do anything  */
             break;
          default :  /*  Possibly valid ASCII character  */
             if (l<length)
             {
                cc=toupper(cc);
                if (charokay(cc,md))
                {
                   buff[l]=cc;
                   cprintf("%c",cc);
                   l++;
                   buff[l]=NULLC;
                }
                else
                   beep();
             }
             else
                beep();
             break;
          }
    }
    */
    if cc == ESCKEY {
        // return -1;
        InputValue::Esc
    } else if buff.is_empty() {
        // return 1;
        InputValue::Blank
    } else {
        // return 0;
        InputValue::InputString(String::from_utf8(buff).unwrap())
    }
} /* End getinp */

/*
void getcourse(void)  /* Gets course and places in variable c */
{
char ibuff[5];
int gb;

   cprintf("COURSE (1-8.99)? ");
   gb=getinp(ibuff,4,2);
   cprintf("\r\n");
   if (gb!=0)
      c=0.0;
   else
      c=atof(ibuff);
}  /* End getcourse */


void getwarp(void)  /* Gets warp and places in variable w */
{
char ibuff[5];
int gb;

   cprintf("WARP (0-12.0)? ");
   gb=getinp(ibuff,4,2);
   cprintf("\r\n");
   if (gb!=0)
      w=0.0;
   else
      w=atof(ibuff);
}  /* End getwarp */
 */
