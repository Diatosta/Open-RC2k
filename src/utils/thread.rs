use libmem::*;
use std::arch::asm;
use windows::Win32::System::Threading::GetCurrentThreadId;

pub fn inject_hooks() {
    let get_thread_offset_params_hk_addr =
        get_thread_offset_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x410CC5, get_thread_offset_params_hk_addr).unwrap();
}

#[naked]
unsafe extern "C" fn get_thread_offset_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push eax", "push ecx", "push edx", "call {}", "mov esi, eax", "pop edx", "pop ecx", "pop eax", "ret", sym get_thread_offset, options(noreturn));
}

// This method seems to be used to have a different buffer for each thread
// And seems to be prepared for a maximum of 4 threads (maybe 5)
pub unsafe fn get_thread_offset() -> usize {
    let current_thread_id = GetCurrentThreadId();

    let thread_id1 = *(0x5184F8 as *mut u32);
    let thread_id2 = *(0x518608 as *mut u32);
    let thread_id3 = *(0x518718 as *mut u32);
    let thread_id4 = *(0x518824 as *mut u32);

    let current_thread_offset = match current_thread_id {
        thread if thread == thread_id1 => 0x5184F8,
        thread if thread == thread_id2 => 0x518608,
        thread if thread == thread_id3 => 0x518718,
        thread if thread == thread_id4 => 0x518824,
        _ => 0,
    };

    // TODO: return this as a value, not as a register
    asm!("test {}, {}", in(reg) current_thread_offset, in(reg) current_thread_offset);

    current_thread_offset
}
