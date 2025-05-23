use libmem::{hook_code, Address};
use std::arch::{asm, naked_asm};
use windows::{
    core::{PCSTR, PSTR},
    Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        Storage::FileSystem::{FILE_END, INVALID_SET_FILE_POINTER},
    },
};

use crate::config::ral_cfg::RAL_CFG_PROPERTIES;
use crate::{filesystem, utils};

static mut WRITING_TO_LOG: bool = false;
static mut IS_NEW_LOG_LINE: bool = false;

pub unsafe fn inject_hooks() { unsafe {
    let set_writing_log_parameters_hk_addr = set_writing_log_parameters as *const () as Address;
    let log_type_1_parameters_hk_addr = log_type_1_parameters as *const () as Address;
    let log_type_2_parameters_hk_addr = log_type_2_parameters as *const () as Address;

    let _ = hook_code(0x40130A, set_writing_log_parameters_hk_addr).unwrap();
    let _ = hook_code(0x4013CB, log_type_1_parameters_hk_addr).unwrap();

    // TODO: Unfortunately log_type_2, 3, 4 and 5 get overlapped by the hook function
    // So we'll only be able to hook them after every call to them is replaced
    //let _ = hook_code(0x401472, log_type_2_parameters_hk_addr).unwrap();
}}

#[unsafe(naked)]
unsafe extern "C" fn set_writing_log_parameters() {
    // We must push and pop all registers as they are needed further on
    naked_asm!("push eax", "call {}", "pop eax", "ret", sym set_writing_log);
}

// Currently this method will always return true, as the compiler seems to think log_file is always true
unsafe fn set_writing_log() { unsafe {
    let log_file = *(0x4E0098 as *mut bool);

    if log_file {
        // Tbh I have no idea what this does, and it might not even be needed
        // But it's here just in case
        WRITING_TO_LOG = true;
        *(0x4E009C as *mut bool) = true;
    }
}}

unsafe fn write_to_log_file(file_pattern: PSTR, file_buffer: PCSTR) { unsafe {
    let file_handle = filesystem::open_or_create_file_hooked(file_pattern.as_ptr(), 4);
    if file_handle == INVALID_HANDLE_VALUE {
        return;
    }

    if filesystem::set_file_pointer_hooked(0, file_handle, FILE_END) != INVALID_SET_FILE_POINTER {
        let _ = filesystem::write_file(file_handle, file_buffer);
    }

    filesystem::close_file(file_handle);
}}

#[unsafe(naked)]
unsafe extern "C" fn log_type_1_parameters() {
    // We must push and pop all registers as they are needed further on
    naked_asm!("push edx", "push edi", "push ecx", "push edi", "movzx ecx, dh", "movzx edi, dl", "push edi", "push ecx", "add esp, 8", "pop ecx", "pop edi", "sub esp, 16", "push eax", "call {}", "add esp, 20", "pop edi", "pop edx", "mov ecx, eax", "ret", sym log_type_1);
}

unsafe fn log_type_1(file_buffer_ptr: *mut u8, log_level: u8, finished: bool) -> u32 { unsafe {
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

    //println!("Type 1: {}", file_buffer);

    result
}}

#[unsafe(naked)]
unsafe extern "C" fn log_type_2_parameters() {
    // We must push and pop all registers as they are needed further on
    naked_asm!("push edx", "push edi", "push ecx", "push eax", "movzx ecx, dh", "movzx eax, dl", "push eax", "push ecx", "add esp, 8", "pop ecx", "pop eax", "sub esp, 16", "call {}", "add esp, 16", "pop edi", "pop edx", "mov ecx, eax", "ret", sym log_type_2);
}

unsafe fn log_type_2(log_level: u8, finished: bool, unk: i32) -> u32 { unsafe {
    if let Some(mut file_buffer) = log_variable(unk) {
        let result = log(&mut file_buffer, log_level, finished);

        //println!("Type 2: {}", file_buffer);

        result
    } else {
        0
    }
}}

#[inline(never)]
unsafe fn log_variable(mut eax: i32) -> Option<String> { unsafe {
    let unk_byte = *(0x4E0094 as *mut u8); // Probably indicates if logging is enabled

    // This is required while not all references to this variable are replaced
    IS_NEW_LOG_LINE = *(0x4E00A4 as *mut bool);

    let mut log_constructor_buffer = String::new();

    if unk_byte == 0 {
        return None;
    }

    set_writing_log();

    if eax < 0 {
        utils::string::insert_dash(&mut log_constructor_buffer);
        eax = -eax;
    }

    // TODO: We don't seem to be able to retrieve the modified string from here, fix it
    sub_40204a(eax as u32, 0, &mut log_constructor_buffer);

    Some(log_constructor_buffer)
}}

// This method stores the division of a1 and a2 in a3, and returns the remainder
// Not sure what to call it, so I left IDA's default name
#[inline(never)]
unsafe fn sub_402112(a1: u32, a2: u32, a3: *mut u8) -> u32 { unsafe {
    *a3 += (a1 / a2) as u8;

    a1 % a2
}}

// Not sure what to call it, so I left IDA's default name
#[inline(never)]
unsafe fn sub_40204a(a1: u32, a2: u8, buffer: &mut String) { unsafe {
    let mut result_arr = [48u8; 11];
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

    let result_str = String::from_utf8_lossy(&result_arr[index..]).to_string();

    utils::string::append(buffer, &result_str);
}}

#[inline(never)]
unsafe fn log(file_buffer: &mut String, log_level: u8, finished: bool) -> u32 { unsafe {
    let mut result = 0;

    let properties = match RAL_CFG_PROPERTIES.try_lock() {
        Ok(properties) => properties,
        Err(e) => {
            println!("Failed to lock RAL_CFG_PROPERTIES to log: {}", e);
            return result;
        }
    };

    if properties.log_file > log_level as i32 {
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

        result = (file_buffer.len() - 1) as u32;
    }

    WRITING_TO_LOG = false;
    *(0x4E009C as *mut bool) = false;

    result
}}
