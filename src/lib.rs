use libmem::*;
use std::{arch::asm, ffi::c_char, mem, path::Path, thread};
use windows::{
    core::PCSTR,
    imp::{GetProcAddress, LoadLibraryA},
    Win32::{
        Foundation::{BOOL, HMODULE, HWND},
        Storage::FileSystem::{FindFileHandle, FindFirstFileA, FindNextFileA, WIN32_FIND_DATAA},
        System::{
            Environment::GetCurrentDirectoryA,
            Registry::{
                RegCloseKey, RegOpenKeyA, RegOpenKeyExA, RegQueryValueExA, HKEY,
                HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, REG_BINARY, REG_SZ, REG_VALUE_TYPE,
            },
        },
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

// This method is used to check if the game is installed to the correct folder (among possibly other things)
// It sets ZF to 1 if the game is installed to the correct folder, and 0 otherwise
// As such, force ZF to 1 to skip this check
unsafe fn sub_401ede_fn() {
    asm!("xor eax, eax",);
}

unsafe fn sub_4030d1_fn() {
    let mut lpbuffer: [u8; 0xFF] = [0; 0xFF];
    let result = GetCurrentDirectoryA(Some(&mut lpbuffer));

    if result != 0 {
        let path = Path::new(std::str::from_utf8_unchecked(&lpbuffer)).to_path_buf();
        let path = if path.is_dir() { path } else { path.join("\\") };
        let path_str = path.to_str().unwrap();
        let path_bytes = path_str.as_bytes();
        lpbuffer[..path_bytes.len()].copy_from_slice(path_bytes);
    }

    asm!(
        "lea edx, [{}]",
        in(reg) &lpbuffer as *const _,
    );
}

// Gets the status and source keys from the registry
unsafe fn maybe_get_registry_game_status_fn() {
    const SOFTWARE_MAGNETIC_FIELDS: &str = "SOFTWARE\\Magnetic Fields\\RC99\x00";
    const STATUS: &str = "status\x00";
    const SOURCE: &str = "source\x00";

    let mut lp_type: REG_VALUE_TYPE = REG_VALUE_TYPE::default();
    let mut registry_status = 0;
    let mut registry_source = vec![0u8; 0xFF];
    let mut lp_cb_data: u32;
    let mut phk_result: HKEY = HKEY::default();

    let mut result = RegOpenKeyExA(
        HKEY_LOCAL_MACHINE,
        PCSTR(SOFTWARE_MAGNETIC_FIELDS.as_ptr()),
        0,
        KEY_READ | KEY_WOW64_32KEY,
        &mut phk_result,
    );
    if result.is_err() {
        return;
    }

    lp_cb_data = 1;
    result = RegQueryValueExA(
        phk_result,
        PCSTR(STATUS.as_ptr()),
        None,
        Some(&mut lp_type),
        Some(&mut registry_status),
        Some(&mut lp_cb_data),
    );

    if result.is_ok() {
        if lp_type != REG_BINARY || lp_cb_data != 1 {
            registry_status = 0;
        }

        lp_cb_data = 0xFF;

        result = RegQueryValueExA(
            phk_result,
            PCSTR(SOURCE.as_ptr()),
            None,
            Some(&mut lp_type),
            Some(registry_source.as_mut_ptr()),
            Some(&mut lp_cb_data),
        );
        if result.is_ok() && lp_type != REG_SZ {
            registry_source.clear();
        }
    }

    RegCloseKey(phk_result);

    // Set 0x51BB8C to registry_status
    *(0x51BB8C as *mut u8) = registry_status;

    // Set 0x51BB8D to registry_source (and copy all data)
    (0x51BB8D as *mut u8).copy_from(registry_source.as_mut_ptr(), 0xFF);
}

unsafe fn maybe_find_file_fn() -> u32 {
    // Set a1 to EAX
    // Ideally this would be done with libmem, but it seems to be broken for now
    let mut a1: i32;
    asm!("mov {a1}, edx", a1 = out(reg) a1);

    // Set 0x4F52B0 to a pointer to a1
    // Ideally this would be done with libmem, but it seems to be broken for now
    *(0x4F52B0 as *mut i32) = a1;

    // Create a pointer with type WIN32_FIND_DATAA that points to 0x4E05A8
    // This is the same as the type of the second argument of FindFirstFileA
    // Ideally this would be done with libmem, but it seems to be broken for now
    let find_file_data = 0x4E05A8 as *mut WIN32_FIND_DATAA;
    let h_find_file = FindFirstFileA(*(0x4F52B0 as *mut PCSTR), find_file_data);

    let result: u32;

    if let Ok(h_find_file_raw) = h_find_file {
        // Copy h_find_file_raw to 0x4E05A4
        // Ideally this would be done with libmem, but it seems to be broken for now
        *(0x4E05A4 as *mut isize) = h_find_file_raw.0;

        result = (*find_file_data).dwFileAttributes;

        if ((*find_file_data).dwFileAttributes & (*find_file_data).cAlternateFileName[14] as u32)
            != 0
        {
            return maybe_find_next_file() as u32;
        }
    } else {
        *(0x4E05A4 as *mut isize) = -1;
        result = 0;
    }

    result
}

unsafe fn maybe_find_next_file() -> i32 {
    let mut result: BOOL;
    let find_file_data = 0x4E05A8 as *mut WIN32_FIND_DATAA;

    // Copy 0x4E05A4 to h_find_file_raw
    // Ideally this would be done with libmem, but it seems to be broken for now
    let h_find_file_raw = *(0x4E05A4 as *const FindFileHandle);

    loop {
        result = FindNextFileA(h_find_file_raw, 0x4E05A8 as *mut WIN32_FIND_DATAA);
        if !result.as_bool() {
            break;
        }
        result = BOOL((*find_file_data).dwFileAttributes as i32);

        if ((*find_file_data).dwFileAttributes & (*find_file_data).cAlternateFileName[14] as u32)
            == 0
        {
            break;
        }
    }

    result.0
}

fn inject_stuff() {
    let sub_401ede_hk_addr = sub_401ede_fn as *const () as lm_address_t;
    let get_registry_game_status_hk_addr = maybe_get_registry_game_status_fn as *const () as lm_address_t;
    let sub_4030d1_hk_addr = sub_4030d1_fn as *const () as lm_address_t;
    let maybe_find_file_hk_addr = maybe_find_file_fn as *const () as lm_address_t;

    //let _ = LM_HookCode(0x401EDE, sub_401ede_hk_addr).unwrap();
    let _ = LM_HookCode(0x413D14, get_registry_game_status_hk_addr).unwrap();
    //let _ = LM_HookCode(0x4030D1, sub_4030d1_hk_addr).unwrap();
    //let _ = LM_HookCode(0x402FC2, maybe_find_file_hk_addr).unwrap();
}

#[no_mangle]
extern "C" fn DirectInputCreateA() -> u32 {
    unsafe {
        let original_dll = LoadLibraryA(PCSTR("C:\\WINDOWS\\SysWOW64\\DINPUT.dll\x00".as_ptr()));
        if original_dll == 0 {
            return 0;
        }

        let original_function: extern "C" fn() -> u32 = mem::transmute(GetProcAddress(
            original_dll,
            PCSTR("DirectInputCreateA\x00".as_ptr()),
        ));

        original_function()
    }
}

#[no_mangle]
extern "system" fn DllMain(_module_handle: HMODULE, dw_reason: u32, _lp_reserved: &u32) -> BOOL {
    match dw_reason {
        1u32 => {
            thread::spawn(inject_stuff);
        }
        _ => return BOOL(0),
    };

    BOOL(1)
}
