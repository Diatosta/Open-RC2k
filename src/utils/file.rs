use libmem::*;
use std::arch::asm;
use super::thread;
use super::string;

pub fn inject_hooks() {
    let build_file_pattern_params_hk_addr = build_file_pattern_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x403206, build_file_pattern_params_hk_addr).unwrap();
}

#[naked]
unsafe extern "C" fn build_file_pattern_parameters() {
    asm!("push ecx", "push edx", "push esi", "push eax", "call {}", "add esp, 8", "pop edx", "pop ecx", "ret", sym build_file_pattern, options(noreturn));
}

unsafe fn build_file_pattern(string: *mut u8) -> *mut u8 {
    let first_dword = std::str::from_utf8(&*(string as *mut [u8; 4]));
    let first_word = std::str::from_utf8(&*(string as *mut [u8; 2]));
    let first_char = std::str::from_utf8(&*(string as *mut [u8; 1]));
    let second_char = std::str::from_utf8(&*(string.add(1) as *mut [u8; 1]));
    let unk_byte = *(0x5189A4 as *mut u8);

    if let (Ok(first_dword), Ok(first_word), Ok(first_char), Ok(second_char)) = (first_dword, first_word, first_char, second_char) {
        if first_dword.eq("var\\") || first_dword.eq("save") || second_char.eq(":") {
            return string;
        }

        let mut edx: *mut u8;
        let mut eax = string;

        if !first_word.eq(";4") && unk_byte <= 3 && !first_word.eq(";3") && unk_byte >= 2 {
            edx = 0x5295B4 as *mut u8;
        } else {
            edx = 0x5189A8 as *mut u8;
        }

        if first_char.eq(";") {
            eax = eax.add(2);
        }

        (eax, edx) = (edx, eax);

        let mut file_pattern_buffer = 0x94E9B0 as *mut u8;

        let thread_offset = thread::get_thread_offset();
        if thread_offset != 0 {
            file_pattern_buffer = (thread_offset + 16) as *mut u8;
        }

        let file_pattern_buffer_start = file_pattern_buffer;

        string::append(file_pattern_buffer, eax);

        loop {
            let mut current_char: u8;

            loop {
                current_char = *edx;
                edx = edx.add(1);

                if current_char != 0x25 /* '%' */ {
                    break;
                }

                string::append(file_pattern_buffer, 0x518AA8 as *mut u8);
            }

            *file_pattern_buffer = current_char;
            file_pattern_buffer = file_pattern_buffer.add(1);

            if current_char == 0 {
                break;
            }
        }

        return file_pattern_buffer_start;
    }

    string
}
