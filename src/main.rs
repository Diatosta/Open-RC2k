#![feature(naked_functions)]
#![feature(lazy_cell)]

mod constants;
mod filesystem;
mod utils;

fn main() {
    let current_system_time = utils::datetime::get_current_system_time();
    let volume_information = filesystem::get_volume_information();

    /*
      getCpuInfo(v5);
      getKeyboardType();
      getWindowsVersion();
      CoInitialize(0);
      sub_40298A((int (**)(void))ThreadOffset, (DWORD)a3);
      setErrorMode();
      loadRalCfg((DWORD)a3, (char *)ThreadOffset);
      setCpuExtensions();
      writeInitialLog((DWORD)a3, (int)ThreadOffset);
      if ( hasCpuFpu == 1 )
      {
        maybe_parseCmdLineArgs(a1);
        v10 = sub_416653(v9, (int)a1, a2, (DWORD)a3, (int)ThreadOffset);
        v13 = sub_416477(v10, v11, v12, a1, a2, (int)a3, ThreadOffset);
        sub_416292(v15, v13, v14);
        loadLangCfg((DWORD)a3, (char *)ThreadOffset);
        loadRalZog((DWORD)a3, (char *)ThreadOffset);
        v19 = sub_40331F(v16, v17, v18, (int *)a1, a2, a3, (int *)ThreadOffset);
        sub_411239(v19, a2, (char **)a3, (int *)ThreadOffset);
        sub_4103BE();
        if ( v21 )
          goto LABEL_6;
        LODWORD(v22) = sub_4103DE(v20, (int)a1, a2, (int)a3, (int (**)(void))ThreadOffset);
        v24 = sub_404981(v22, v23, (int)a1);
        sub_404270();
        sub_4043DA(SHIDWORD(v24), v25, (int)a1, (int)a3, (int (**)(void))ThreadOffset);
        sub_404401(v24, v26, (int)a1, (int)a3, (int (**)(void))ThreadOffset);
        sub_41848E(v24, a2, (int)a3, (int (**)(void))ThreadOffset);
        sub_410B68(0x80u);
        v27 = sub_410B84(128);
        sub_409C1E(v27, (int)v7, v28, a2, (int)a3, (int)ThreadOffset);
        v6 = 1000;
        sub_41100C(0x3E8u);
        sub_404481(a1, a2, (int)a3, (int (**)(void))ThreadOffset);
        if ( !v21 )                                 // Seems to check video capabilities
        {
          sub_40608F(1000, v7, a2, (char **)a3, (int *)ThreadOffset);
          sub_405DB4();
          sub_405580();
          sub_407062();
          sub_40F9DE();
          v31 = sub_415383(v30, v29);
    LABEL_5:
          sub_4184D6(SHIDWORD(v31), (int (**)(void))ThreadOffset);
          sub_404CD6(v32, SHIDWORD(v31));
          sub_40FA8B();
          sub_4060B7(SHIDWORD(v31), (int (**)(void))ThreadOffset);
          sub_410509();
          sub_410B68(0x20u);
          sub_412FA2(v33);
    LABEL_6:
          sub_402B95();
          sub_4029DB();
          CoUninitialize();
          sub_407855(v34);
          ExitProcess(Msg.wParam);
        }
        dword_4F52A4 = -1876946687;
        dword_4F52A8 = (int)"Required 2D/3D video modes not found (acceleration/d3d disabled, unsuitable card or not enough v"
                            "ideo memory)\\nPlease check your video card installation\\nNote an 8Mb 3D video card is required"
                            " (4Mb display+4Mb texture memory)";
      }
      else
      {
        dword_4F52A4 = -1879043841;
        dword_4F52A8 = (int)"FPU is required\t";
      }
      v61 = ThreadOffset;
      v59 = v8;
      v58 = (int)v7;
      v57 = v6;
      __readeflags();
      v35 = byte_4F67BC;
      byte_4F67BC = 1;
      if ( v35 != 1 )
      {
        v36 = Text;
        if ( (dword_4F52A4 & 0x10000000) == 0 )
        {
          v37 = aBreakpoint;
          if ( dword_4F52A4 < 0 )
            v37 = aErrorCode;
          stringAppend(v37, Text);
          *(_WORD *)Text = 23328;
          sub_40211D(dword_4F52A4, 8, &Text[2]);
          *(_DWORD *)&Text[2] = 658781;
          v36 = &Text[5];
        }
        v38 = (CHAR *)dword_4F52A8;
        if ( dword_4F52A8 && *(_BYTE *)dword_4F52A8 )
        {
          while ( 1 )
          {
            while ( 1 )
            {
              v39 = *v38++;
              if ( v39 != 92 || *v38 != 110 )
                break;
              ++v38;
              *(_WORD *)v36 = 2573;
              v36 += 2;
            }
            if ( !v39 )
              break;
            *v36++ = v39;
          }
          *(_WORD *)v36 = 2573;
          v36 += 2;
        }
        stringAppend(byte_4F67BD, v36);
        if ( (dword_4F52A4 & 0x10000000) == 0 )
        {
          if ( currentFilePattern )
          {
            v56 = (char *)currentFilePattern;
            stringAppend(aLastFile, v36);
            *(_WORD *)v36 = 24608;
            v41 = v36 + 2;
            stringAppend(v56, v41);
            *(_DWORD *)v41 = 658727;
            v36 = v41 + 3;
          }
          if ( dword_4E0598 )
          {
            stringAppend(aW32, v36);
            sub_40211D(v42, 8, v36);
            *(_WORD *)v36 = 2573;
            v36 += 2;
          }
          sub_40211D((int)v61, 8, &aEaxXxxxxxxxEbx[4]);
          sub_40211D((int)&retaddr, v43, &aEaxXxxxxxxxEbx[9]);
          sub_40211D((int)a3, v44, &aEaxXxxxxxxxEbx[14]);
          sub_40211D(a2, v45, &aEaxXxxxxxxxEbx[19]);
          sub_40211D(v58, v46, &aEaxXxxxxxxxEbx[25]);
          sub_40211D(v57, v47, &aEaxXxxxxxxxEbx[30]);
          sub_40211D((int)a1, v48, &aEaxXxxxxxxxEbx[35]);
          sub_40211D(v59, v49, &aEaxXxxxxxxxEbx[40]);
          sub_40201D((v40 & 1) != 0, 1, &aEaxXxxxxxxxEbx[44]);
          sub_40201D((v40 & 0x40) != 0, v50, &aEaxXxxxxxxxEbx[48]);
          sub_40201D((v40 & 0x80) != 0, v51, &aEaxXxxxxxxxEbx[52]);
          sub_40201D((v40 & 0x800) != 0, v52, &aEaxXxxxxxxxEbx[56]);
          sub_40201D((v40 & 4) != 0, v53, &aEaxXxxxxxxxEbx[60]);
          stringAppend(aEaxXxxxxxxxEbx, v36);
        }
        maybe_AddNullTerminator(v36);
        dword_4E0094 = 1;
        sub_40142B((DWORD)a3, (int)v38);
        sub_40142B((DWORD)a3, (int)v38);
      }
      result = v57;
      if ( dword_4F52A4 < 0 )
      {
        sub_410BF9();
        ThreadOffset = (BYTE *)getThreadOffset();
        if ( v55 )
          goto LABEL_5;
        return (*(int (**)(void))ThreadOffset)();
      }
      return result;
         */
}
