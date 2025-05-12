mod config;
mod constants;
mod filesystem;
mod logging;
mod utils;

use std::arch::naked_asm;
use std::{arch::asm, mem};
use windows::core::BOOL;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HMODULE,
        System::{
            Console::AllocConsole,
            LibraryLoader::{GetProcAddress, LoadLibraryA},
        },
    },
};
pub use {config::*, constants::*, filesystem::*, utils::*};

pub fn inject_hooks() {
    unsafe {
        filesystem::inject_hooks();
        logging::inject_hooks();
        utils::inject_hooks();
        config::inject_hooks();
    }
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn DirectInputCreateA() -> () {
    // We have to fix the stack manually, as the generated function seems to screw it up
    naked_asm!("call {}", "add esp, 20", "push [esp - 20]", "ret", sym direct_input_create_a_impl);
}

unsafe fn direct_input_create_a_impl(
    _: usize,
    h_instance: usize,
    dw_version: usize,
    pp_direct_input_a: usize,
    p_unk_outer: usize,
) -> u32 { unsafe {
    match LoadLibraryA(PCSTR("C:\\WINDOWS\\SysWOW64\\DINPUT.dll\x00".as_ptr()))
    { Ok(original_dll) => {
        let original_function: extern "C" fn(usize, usize, usize, usize) -> u32 = mem::transmute(
            GetProcAddress(original_dll, PCSTR("DirectInputCreateA\x00".as_ptr())),
        );

        // Pass the opposite way, as for some reason it gets switched
        let result = original_function(h_instance, dw_version, pp_direct_input_a, p_unk_outer);

        asm!("sub esp, 16");

        result
    } _ => {
        0
    }}
}}

#[unsafe(no_mangle)]
unsafe extern "system" fn DllMain(
    _module_handle: HMODULE,
    dw_reason: u32,
    _lp_reserved: &u32,
) -> BOOL {
    match dw_reason {
        1u32 => {
            std::thread::spawn(inject_hooks);
            unsafe {
                let _ = AllocConsole();
            }
        }
        _ => return BOOL(0),
    };

    BOOL(1)
}
