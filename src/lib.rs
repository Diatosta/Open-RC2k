#![feature(naked_functions)]

use libmem::*;
use std::{
    arch::asm,
    ffi::{c_char, CStr},
    mem, thread,
};
use windows::{
    core::PCSTR,
    imp::{GetProcAddress, LoadLibraryA},
    Win32::{
        Foundation::{GetLastError, BOOL, HMODULE},
        Storage::FileSystem::{FindFileHandle, FindFirstFileA, FindNextFileA, WIN32_FIND_DATAA},
        System::{
            Environment::GetCurrentDirectoryA,
            Registry::{
                RegCloseKey, RegOpenKeyExA, RegQueryValueExA, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
                KEY_WOW64_32KEY, REG_BINARY, REG_SZ, REG_VALUE_TYPE,
            },
        },
    },
};

// This method is used to check if the game is installed to the correct folder (among possibly other things)
// It sets ZF to 1 if the game is installed to the correct folder, and 0 otherwise
// As such, force ZF to 1 to skip this check
unsafe fn sub_401ede_fn() {
    asm!("xor eax, eax",);
}

// Gets the current directory
unsafe fn maybe_get_current_directory_fn() {
    let mut current_directory: [u8; 0xFF] = [0; 0xFF];
    let bytes_written = GetCurrentDirectoryA(Some(&mut current_directory));

    if bytes_written != 0 {
        // Convert current_directory to a String
        let mut current_directory_str =
            std::str::from_utf8_unchecked(&current_directory[..bytes_written as usize]).to_string();

        if !current_directory_str.ends_with('\\') && !current_directory_str.ends_with('/') {
            current_directory_str.push('\\');
            current_directory[..current_directory_str.len()]
                .copy_from_slice(current_directory_str.as_bytes());
        }
    }

    asm!(
        "lea edx, [{}]",
        in(reg) &current_directory as *const _,
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

#[naked]
unsafe extern "C" fn maybe_find_file_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    asm!("push edx", "push eax", "call {}", "add esp, 8", "ret", sym maybe_find_file_impl, options(noreturn));
}

unsafe fn maybe_find_file_impl(a1: *const u8, edx: u32) -> u32 {
    let lp_filename = PCSTR::from_raw(a1);

    // Create a pointer with type WIN32_FIND_DATAA that points to 0x4E05A8
    // This is the same as the type of the second argument of FindFirstFileA
    // Ideally this would be done with libmem, but it seems to be broken for now
    let find_file_data = 0x4E05A8 as *mut WIN32_FIND_DATAA;
    let h_find_file = FindFirstFileA(lp_filename, find_file_data);

    let result: u32;

    match h_find_file {
        Ok(h_find_file_raw) => {
            // Copy h_find_file_raw to 0x4E05A4
            *(0x4E05A4 as *mut isize) = h_find_file_raw.0;

            let unk_value = *(0x4E06E6 as *const u32);

            let flags = (*find_file_data).dwFileAttributes & unk_value;

            if flags != 0 {
                result = maybe_find_next_file() as u32;
            } else {
                result = (*find_file_data).dwFileAttributes;
            }
        }
        Err(_) => {
            *(0x4E05A4 as *mut isize) = -1;
            result = 0;
        }
    }

    // Restore edx
    asm!("mov edx, {}", in(reg) edx);

    result
}

unsafe fn maybe_find_next_file() -> i32 {
    let mut result: BOOL;
    let find_file_data = 0x4E05A8 as *mut WIN32_FIND_DATAA;
    let mut unk_value: u32;

    // Copy 0x4E05A4 to h_find_file_raw
    // Ideally this would be done with libmem, but it seems to be broken for now
    let h_find_file_raw = *(0x4E05A4 as *const FindFileHandle);

    loop {
        result = FindNextFileA(h_find_file_raw, 0x4E05A8 as *mut WIN32_FIND_DATAA);
        if !result.as_bool() {
            break;
        }
        result = BOOL((*find_file_data).dwFileAttributes as i32);

        unk_value = *(0x4E06E6 as *const u32);

        let flags = (*find_file_data).dwFileAttributes & unk_value;

        if flags == 0 {
            break;
        }
    }

    result.0
}

#[naked]
unsafe extern "C" fn sub_403070_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    asm!("push ebx", "push edx", "call {}", "mov ebx, eax", "add esp, 8", "ret", sym sub_403070, options(noreturn));
}

unsafe fn sub_403070(a1: *const c_char, a2: *mut u8) -> usize {
    // Convert a1 pointer to a &str
    let a1_str = CStr::from_ptr(a1).to_str();

    if let Ok(a1_str) = a1_str {
        // Copy a1 to a2 until the last occurence of a '\\', '/' or ':'
        let temp_str = a1_str.rsplit_once(|c| c == '\\' || c == '/' || c == ':');

        if let Some((temp_str, _)) = temp_str {
            a2.copy_from(temp_str.as_ptr(), temp_str.len() + 1); // + 1 for the null terminator

            // Move the end of string to buffer it was moved into as it'll be needed later
            return a2 as usize + temp_str.len() + 1;
        }
    }

    0
}

fn inject_stuff() {
    let sub_401ede_hk_addr = sub_401ede_fn as *const () as lm_address_t;
    let maybe_get_registry_game_status_hk_addr =
        maybe_get_registry_game_status_fn as *const () as lm_address_t;
    let maybe_get_current_directory_hk_addr =
        maybe_get_current_directory_fn as *const () as lm_address_t;
    let maybe_find_file_params_hk_addr = maybe_find_file_parameters as *const () as lm_address_t;
    let sub_403070_params_hk_addr = sub_403070_parameters as *const () as lm_address_t;

    //let _ = LM_HookCode(0x401EDE, sub_401ede_hk_addr).unwrap();
    let _ = LM_HookCode(0x413D14, maybe_get_registry_game_status_hk_addr).unwrap();
    //let _ = LM_HookCode(0x4030D1, maybe_get_current_directory_hk_addr).unwrap();
    let _ = LM_HookCode(0x402FC2, maybe_find_file_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x403070, sub_403070_params_hk_addr).unwrap();
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
