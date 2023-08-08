use std::{mem, ptr, thread};
use windows::{
    core::PCSTR,
    imp::{GetProcAddress, LoadLibraryA},
    Win32::{
        Foundation::{BOOL, HMODULE, HWND},
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

const SUB401EDE: usize = 0x401EDE;

fn inject_stuff() {
    unsafe {
        MessageBoxA(
            HWND(0),
            PCSTR("DLL Hijacked!\x00".as_ptr()),
            PCSTR("Uh oh\x00".as_ptr()),
            MESSAGEBOX_STYLE(0),
        );

        let module_handle = GetModuleHandleA(PCSTR(ptr::null())).unwrap();
        let handle = format!("module_handle: {:x}\x00", module_handle.0);
        MessageBoxA(
            HWND(0),
            PCSTR(handle.as_ptr()),
            PCSTR("Uh oh\x00".as_ptr()),
            MESSAGEBOX_STYLE(0),
        );
    };
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
            std::thread::spawn(inject_stuff);
        }
        _ => return BOOL(0),
    };

    BOOL(1)
}
