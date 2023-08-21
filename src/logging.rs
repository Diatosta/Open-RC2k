use libmem::*;
use std::arch::asm;
use windows::Win32::{Foundation::INVALID_HANDLE_VALUE, Storage::FileSystem::{FILE_END, INVALID_SET_FILE_POINTER}};

use crate::filesystem;

static mut WRITING_TO_LOG: bool = false;

pub fn inject_hooks() {
    let is_logging_enabled_parameters_hk_addr =
        is_logging_enabled_parameters as *const () as lm_address_t;
    let write_to_log_parameters_hk_addr = write_to_log_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x40130A, is_logging_enabled_parameters_hk_addr).unwrap();
    let _ = LM_HookCode(0x402C9C, write_to_log_parameters_hk_addr).unwrap();
}

#[naked]
unsafe extern "C" fn is_logging_enabled_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push eax", "call {}", "pop eax", "ret", sym is_logging_enabled, options(noreturn));
}

// Currently this method will always return true, as the compiler seems to think log_file is always true
unsafe fn is_logging_enabled() {
    let log_file = *(0x4E0098 as *mut bool);

    if log_file {
        // Tbh I have no idea what this does, and it might not even be needed
        // But it's here just in case
        WRITING_TO_LOG = true;
    }
}

#[naked]
unsafe extern "C" fn write_to_log_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("pusha", "push ebx", "push ecx", "push eax", "call {}", "add esp, 12", "popa", "ret", sym write_to_log, options(noreturn));
}

unsafe fn write_to_log(file_pattern: *mut u8, number_of_bytes_to_write: u32, file_buffer: *mut u8) {
    let file_handle = filesystem::open_or_create_file(file_pattern, 4);
    if file_handle == INVALID_HANDLE_VALUE {
        return;
    }

    if filesystem::set_file_pointer(0, file_handle, FILE_END) != INVALID_SET_FILE_POINTER {
        filesystem::write_file(number_of_bytes_to_write, file_handle, file_buffer);
    }

    filesystem::close_file(file_handle);
}
