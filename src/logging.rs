use libmem::*;
use std::arch::asm;

static mut WRITING_TO_LOG: bool = false;

pub fn inject_hooks() {
    let is_logging_enabled_parameters_hk_addr =
        is_logging_enabled_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(
        0x40130A,
        is_logging_enabled_parameters_hk_addr,
    )
    .unwrap();
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
