use libmem::*;
use std::{arch::asm, mem, thread};
use windows::{
    core::PCSTR,
    imp::{GetProcAddress, LoadLibraryA},
    Win32::Foundation::{BOOL, HMODULE},
};

// This method is used to check if the game is installed to the correct folder (among possibly other things)
// It sets ZF to 1 if the game is installed to the correct folder, and 0 otherwise
// As such, force ZF to 1 to skip this check
unsafe fn sub_401ede_fn() {
    asm!("xor eax, eax",);
}

fn inject_stuff() {
    let hk_addr = sub_401ede_fn as *const () as lm_address_t;

    let _ = LM_HookCode(0x401EDE, hk_addr).unwrap();
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
