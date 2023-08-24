use libmem::*;
use std::arch::asm;
use windows::{
    core::{PCSTR, PSTR},
    Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        Storage::FileSystem::{FILE_END, INVALID_SET_FILE_POINTER},
    },
};

use crate::{filesystem, utils};

static mut WRITING_TO_LOG: bool = false;
static mut IS_NEW_LOG_LINE: bool = false;

pub fn inject_hooks() {
    let set_writing_log_parameters_hk_addr =
        set_writing_log_parameters as *const () as lm_address_t;
    let log_type_1_parameters_hk_addr = log_type_1_parameters as *const () as lm_address_t;
    let sub_40204a_params_hk_addr = sub_40204a_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x40130A, set_writing_log_parameters_hk_addr).unwrap();
    let _ = LM_HookCode(0x4013CB, log_type_1_parameters_hk_addr).unwrap();
    let _ = LM_HookCode(0x40204A, sub_40204a_params_hk_addr).unwrap();
}

#[naked]
unsafe extern "C" fn set_writing_log_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push eax", "call {}", "pop eax", "ret", sym set_writing_log, options(noreturn));
}

// Currently this method will always return true, as the compiler seems to think log_file is always true
unsafe fn set_writing_log() {
    let log_file = *(0x4E0098 as *mut bool);

    if log_file {
        // Tbh I have no idea what this does, and it might not even be needed
        // But it's here just in case
        WRITING_TO_LOG = true;
        *(0x4E009C as *mut bool) = true;
    }
}

unsafe fn write_to_log_file(file_pattern: PSTR, file_buffer: PCSTR) {
    let file_handle = filesystem::open_or_create_file(file_pattern.as_ptr(), 4);
    if file_handle == INVALID_HANDLE_VALUE {
        return;
    }

    if filesystem::set_file_pointer(0, file_handle, FILE_END) != INVALID_SET_FILE_POINTER {
        let _ = filesystem::write_file(file_handle, file_buffer);
    }

    filesystem::close_file(file_handle);
}

#[naked]
unsafe extern "C" fn log_type_1_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push edx", "push edi", "push ecx", "push edi", "movzx ecx, dh", "movzx edi, dl", "push edi", "push ecx", "add esp, 8", "pop ecx", "pop edi", "sub esp, 16", "push eax", "call {}", "add esp, 20", "pop edi", "pop edx", "mov ecx, eax", "ret", sym log_type_1, options(noreturn));
}

unsafe fn log_type_1(file_buffer_ptr: *mut u8, log_level: u8, finished: bool) -> u32 {
    let unk_byte = *(0x4E0094 as *mut u8); // Probably indicates if logging is enabled

    // This is required while not all references to this variable are replaced
    IS_NEW_LOG_LINE = *(0x4E00A4 as *mut bool);

    if unk_byte == 0 {
        return 0;
    }

    set_writing_log();

    let mut file_buffer =
        String::from_utf8_lossy(PSTR::from_raw(file_buffer_ptr).as_bytes()).to_string();

    let result = log(&mut file_buffer, log_level, finished);

    println!("Type 1: {}", file_buffer);

    result
}

#[naked]
unsafe extern "C" fn log_type_2_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push edx", "push edi", "push ecx", "push edi", "movzx ecx, dh", "movzx edi, dl", "push edi", "push ecx", "add esp, 8", "pop ecx", "pop edi", "sub esp, 16", "push eax", "call {}", "add esp, 20", "pop edi", "pop edx", "mov ecx, eax", "ret", sym log_type_2, options(noreturn));
}

unsafe fn log_type_2(file_buffer_ptr: *mut u8, log_level: u8, finished: bool, unk: u32) -> u32 {
    let unk_byte = *(0x4E0094 as *mut u8); // Probably indicates if logging is enabled

    // This is required while not all references to this variable are replaced
    IS_NEW_LOG_LINE = *(0x4E00A4 as *mut bool);

    if unk_byte == 0 {
        return 0;
    }

    set_writing_log();

    let mut file_buffer =
        String::from_utf8_lossy(PSTR::from_raw(file_buffer_ptr).as_bytes()).to_string();

    let result = log(&mut file_buffer, log_level, finished);

    println!("Type 1: {}", file_buffer);

    result
}

unsafe fn log_variable() {}

// This method stores the division of a1 and a2 in a3, and returns the remainder
// Not sure what to call it, so I left IDA's default name
unsafe fn sub_402112(a1: u32, a2: u32, a3: *mut u8) -> u32 {
    *a3 += (a1 / a2) as u8;

    a1 % a2
}

unsafe extern "C" fn sub_40204a_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push edx", "push eax", "push ecx", "push ebx", "push ecx", "push eax", "call {}", "add esp, 12", "mov ebx, eax", "pop ecx", "pop eax", "pop edx", "ret", sym sub_40204a, options(noreturn));
}

// Not sure what to call it, so I left IDA's default name
unsafe fn sub_40204a(a1: u32, a2: u8, a3: *mut u8) -> *mut u8 {
    let mut result_arr = [48u8; 12];
    let mut remainder: u32;

    remainder = sub_402112(a1, 1000000000, result_arr.as_ptr().add(1) as *mut u8);
    remainder = sub_402112(remainder, 100000000, result_arr.as_ptr().add(2) as *mut u8);
    remainder = sub_402112(remainder, 0x989680, result_arr.as_ptr().add(3) as *mut u8); // Not sure what this magic number is
    remainder = sub_402112(remainder, 1000000, result_arr.as_ptr().add(4) as *mut u8);
    remainder = sub_402112(remainder, 100000, result_arr.as_ptr().add(5) as *mut u8);
    remainder = sub_402112(remainder, 10000, result_arr.as_ptr().add(6) as *mut u8);
    remainder = sub_402112(remainder, 1000, result_arr.as_ptr().add(7) as *mut u8);
    remainder = sub_402112(remainder, 100, result_arr.as_ptr().add(8) as *mut u8);
    remainder = sub_402112(remainder, 10, result_arr.as_ptr().add(9) as *mut u8);

    result_arr[10] += remainder as u8;
    result_arr[11] = 0;

    let mut index = result_arr.len() - ((a2 & 15) as usize) - 1;


    if (a2 & 16) == 0 {
        if (a2 & 32) != 0 {
            let mut i = result_arr.len() - (a2 & 15) as usize;

            loop {
                if result_arr[i] == b'0' {
                    result_arr[i] = b' ';
                } else {
                    if result_arr[i] != 0 {
                        result_arr[i - 1] = b'0';
                    }

                    break;
                }

                i += 1;
            }
        } else {
            index = 0;

            loop {
                if result_arr[index] != b'0' {
                    if result_arr[index] != 0 {
                        index -= 1;
                    }

                    index += 1;

                    break;
                }

                index += 1;
            }
        }
    }

    utils::string::append(a3, result_arr[index..].as_mut_ptr())
}

unsafe fn log(file_buffer: &mut String, log_level: u8, finished: bool) -> u32 {
    let log_file_level = *(0x4E0098 as *mut u8);
    let mut result = 0;

    if log_file_level > log_level {
        if IS_NEW_LOG_LINE {
            // Add spaces to the beginning of the line
            IS_NEW_LOG_LINE = false;
            *(0x4E00A4 as *mut bool) = false;

            let mut spaces_to_add = *(0x4E00A0 as *mut i8);

            if spaces_to_add < 0 {
                spaces_to_add = 0;
            } else if spaces_to_add > 80 {
                spaces_to_add = 80;
            }

            let spaces_str = " ".repeat(spaces_to_add as usize);

            file_buffer.insert_str(0, &spaces_str);
        }

        if finished {
            utils::string::insert_new_line(file_buffer);
            IS_NEW_LOG_LINE = true;
            *(0x4E00A4 as *mut bool) = true;
        } else {
            utils::string::insert_space(file_buffer);
        }

        let file_pattern = PSTR::from_raw(0x4E00A8 as *mut u8);
        utils::string::insert_null_terminator(file_buffer);

        // Reenable after implementing all calls to log
        //print!("{}", file_buffer.as_str());

        write_to_log_file(file_pattern, PCSTR(file_buffer.as_ptr()));

        result = file_buffer.len() as u32;
    }

    WRITING_TO_LOG = false;
    *(0x4E009C as *mut bool) = false;

    result
}
