pub struct TheGame {}

impl TheGame {
    pub fn new() -> Self {
        Self {}
    }
}
/*
int gamecomp;  /* 0 = in progress; 1 = won; -1 = lost; -99 = quit */
int ,rating,moved;
double drate;

      init();
      gamecomp=FALSE;
      newquad=TRUE;
      while (!gamecomp)
      {
         if (newquad) setupquad();
         newquad=FALSE;
         moved=FALSE;
         srscan();
         if (e<=0.0)  /* Ran out of energy */
            gamecomp=-1;
         else
         {
            while (TRUE)  /* Command loop (-99 or ESC to quit) */
            {
               cprintf("COMMAND? ");
               a=getinp(cmdbuff,7,2);
               cprintf("\r\n");
               if (a==1) a=99;
               if (a==-1) a=-99;
               if (a==0) a=atoi(cmdbuff);
               if (a==-99)
               {
                  cprintf("\r\nARE YOU SURE YOU WANT TO QUIT? ");
                  yesno();
                  if (ans=='Y')
                  {
                     gamecomp=-99;
                     break;  /* Break out of command loop */
                  }
                  else
                     continue;  /* Back to top of command loop */
               }
               if ((a<1)||(a>6))
               {
                  for (i=0;i<6;i++)
                     cprintf("  %i = %s\r\n",i+1,ds[i]);
                  cprintf("  -99 OR ESC TO QUIT\r\n\n");
                  continue;  /* Back to top of command loop */
               }
               switch (a)
               {
                  case 1 :  /* Warp engines */
                     while (TRUE)
                     {
                        while (TRUE)
                        {
                           getcourse();
                           if (c<9.0) break;
                           beep();
                        }
                        if (c>=1.0)
                           while (TRUE)
                           {
                              getwarp();
                              if ((w<=0.0)||(w>12.0))
                              {
                                 c=10.0;
                                 break;
                              }
                              if ((d[0]>0)&&(w>0.2))
                              {
                                 i=0;
                                 cprintf("%s DAMAGED; MAX IS 0.2; ",ds[i]);
                                 showestreptime();
                                 beep();
                              }
                              else
                                 break;
                              beep();
                           }
                        if (c<9.0) break;
                     }
                     if (c<1.0) break;  /* Abort move */
                     checkforhits();
                     if (e<=0.0)  /* Ran out of energy */
                     {
                        gamecomp=-1;
                        break;
                     }
                     if (rnd()<=0.25)
                     {
                        x=floor(rnd()*6.0);
                        if (rnd()<=0.5)
                        {
                           beep();
                           d[x]+=floor(6.0-rnd()*5.0);
                           cprintf("**SPACE STORM, %s DAMAGED**\r\n",ds[x]);
                           i=x;
                           showestreptime();
                           d[x]++;
                           delay(100);
                           beep();
                        }
                        else
                        {
                           j=-1;
                           for (i=x;i<6;i++)
                              if (d[i]>0)
                              {
                                 j=i;
                                 break;
                              }
                           if (j<0)
                              for (i=0;i<x;i++)
                                 if (d[i]>0)
                                 {
                                    j=i;
                                    break;
                                 }
                           if (j>=0)
                           {
                              d[j]=1;
                              cprintf("**SPOCK USED A NEW REPAIR TECHNIQUE**\r\n");
                           }
                        }
                     }
                     for (i=0;i<6;i++)
                        if (d[i]!=0)
                        {
                           d[i]--;
                           if (d[i]<=0)
                           {
                              d[i]=0;
                              cprintf("%s ARE FIXED!\r\n",ds[i]);
                              beep();
                           }
                        }
                     n=floor(w*8.0);
                     e=e-n-n+0.5;
                     t++;
                     sect[s1][s2]=1;
                     if (t>t9)  /* Ran out of time! */
                     {
                        gamecomp=-1;
                        break;
                     }
                     dopath();
                     if (e<=0.0)  /* Ran out of energy */
                     {
                        gamecomp=-1;
                        break;
                     }
                     moved=TRUE;
                     break;

                  case 2 :  /* Short-range scan */
                     srscan();
                     break;

                  case 3 :  /* Long-range scan */
                     lrscan();
                     break;

                  case 4 :  /* Phasers */
                     phasers();
                     if (x>0.0)
                     {
                        if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                        checkforhits();
                        if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                        if (k9<1) gamecomp=1;  /* All Klingons destroyed! */
                        if (!gamecomp) checkcond();
                     }
                     break;

                  case 5 :  /* Photon torpedos */
                     if (d[4]>0)  /* Torpedoes damaged */
                     {
                        cprintf("SPACE CRUD BLOCKING TUBES.  ");
                        i=4;
                        showestreptime();
                        beep();
                        break;
                     }
                     n=15;
                     if (p<1)
                     {
                        cprintf("NO TORPEDOES LEFT!\r\n");
                        break;
                     }
                     c=10.0;
                     while (c>=9.0)
                     {
                        cprintf("TORPEDO ");
                        getcourse();
                     }
                     if (c<1.0) break;  /* Abort firing of torpedo */
                     p--;
                     cprintf("TRACK: ");
                     dopath();
                     if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                     checkforhits();
                     if (e<=0.0) gamecomp=-1;  /* Ran out of energy */
                     if (k9<1) gamecomp=1;  /* All Klingons destroyed! */
                     if (!gamecomp) checkcond();
                     break;

                  case 6 :  /* Galactic records */
                     galrecs();
                     break;
               }
               if (gamecomp) break;
               if (moved) break;  /* Enterprise moved */
            }  /* End command loop */
         }
      }  /* Game is over! */
      showstardate();
      switch (gamecomp)
      {
         case 1 :
            drate=t-t0;
            rating=(k0/drate)*1000.0;
            cprintf("THE FEDERATION HAS BEEN SAVED!\r\n");
            cprintf("YOU ARE PROMOTED TO ADMIRAL.\r\n");
            cprintf("%i KLINGONS IN %i YEARS.  RATING = %i\r\n\n",
               k0,t-t0,rating);
            break;
         case -1 :
            if (t>t9)
               cprintf("YOU RAN OUT OF TIME!\r\n");
            if (e<=0.0)
               cprintf("YOU RAN OUT OF ENERGY!\r\n");
            cprintf("THANKS TO YOUR BUNGLING, THE FEDERATION WILL BE\r\n");
            cprintf("CONQUERED BY THE REMAINING %i KLINGON CRUISERS!\r\n",k9);
            cprintf("YOU ARE DEMOTED TO CABIN BOY!\r\n");
            break;
         case -99 :
            cprintf("OKAY, QUITTER -- NO KUDOS FOR YOU.\r\n");
            break;
      }

 */
