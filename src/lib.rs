use libmem::*;
use std::{arch::asm, mem, thread, ffi::c_char, path::Path};
use windows::{
    core::PCSTR,
    imp::{GetProcAddress, LoadLibraryA},
    Win32::{Foundation::{BOOL, HMODULE, HWND}, UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE}, System::Environment::GetCurrentDirectoryA},
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
        let path = if path.is_dir() {
            path
        } else {
            path.join("\\")
        };
        let path_str = path.to_str().unwrap();
        let path_bytes = path_str.as_bytes();
        lpbuffer[..path_bytes.len()].copy_from_slice(path_bytes);
    }

    asm!(
        "lea edx, [{}]",
        in(reg) &lpbuffer as *const _,
    );
}

unsafe fn get_registry_game_status_fn() {
    let software_magnetic_fields_addr = 0x51BB60 as *const c_char as lm_address_t;
    let software_magnetic_fields_value: [c_char; 20] = LM_ReadMemory(software_magnetic_fields_addr).unwrap();

    // TODO: this is broken, possibly due to a library bug
    // Wait for a fix
    let value_string = format!("{:?}\x00", software_magnetic_fields_value);

    MessageBoxA(
        HWND(0),
        PCSTR(value_string.as_ptr()),
        PCSTR("Uh oh\x00".as_ptr()),
        MESSAGEBOX_STYLE(0),
    );
}

fn inject_stuff() {
    let sub_401ede_hk_addr = sub_401ede_fn as *const () as lm_address_t;
    let get_registry_game_status_hk_addr = get_registry_game_status_fn as *const () as lm_address_t;
    let sub_4030d1_hk_addr = sub_4030d1_fn as *const () as lm_address_t;

    let _ = LM_HookCode(0x401EDE, sub_401ede_hk_addr).unwrap();
    //let _ = LM_HookCode(0x413D14, get_registry_game_status_hk_addr).unwrap();
    let _ = LM_HookCode(0x4030D1, sub_4030d1_hk_addr).unwrap();
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
