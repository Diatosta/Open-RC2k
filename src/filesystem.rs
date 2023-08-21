use libmem::*;
use std::{
    arch::asm,
    ffi::{c_char, CStr, c_void},
};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{CloseHandle, BOOL, HANDLE, INVALID_HANDLE_VALUE},
        Storage::FileSystem::{
            CreateFileA, FindClose, FindFirstFileA, FindNextFileA, SetFilePointer,
            FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_CREATION_DISPOSITION,
            FILE_SHARE_MODE, SET_FILE_POINTER_MOVE_METHOD, WIN32_FIND_DATAA,
        },
        System::{
            Environment::GetCurrentDirectoryA,
            Registry::{
                RegCloseKey, RegOpenKeyExA, RegQueryValueExA, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
                KEY_WOW64_32KEY, REG_BINARY, REG_SZ, REG_VALUE_TYPE,
            },
        },
    },
};
use windows_sys::Win32::Storage::FileSystem::{WriteFile, ReadFile};

use crate::utils::string;
use crate::utils::thread;

static mut H_FIND_FILE: HANDLE = INVALID_HANDLE_VALUE;

pub fn inject_hooks() {
    let get_registry_game_status_hk_addr = get_registry_game_status as *const () as lm_address_t;
    let get_current_directory_params_hk_addr =
        get_current_directory_parameters as *const () as lm_address_t;
    let find_file_params_hk_addr = find_file_parameters as *const () as lm_address_t;
    let get_directory_path_params_hk_addr =
        get_directory_path_parameters as *const () as lm_address_t;
    let set_current_directory_hk_addr = set_current_directory as *const () as lm_address_t;
    let read_file_params_hk_addr = read_file_parameters as *const () as lm_address_t;
    let find_close_params_hk_addr = find_close_parameters as *const () as lm_address_t;
    let is_game_installed_in_current_directory_params_hk_addr =
        is_game_installed_in_current_directory_parameters as *const () as lm_address_t;
    let open_or_create_file_params_hk_addr =
        open_or_create_file_parameters as *const () as lm_address_t;
    let set_file_pointer_params_hk_addr = set_file_pointer_parameters as *const () as lm_address_t;
    let build_file_pattern_params_hk_addr =
        build_file_pattern_parameters as *const () as lm_address_t;
    let write_file_params_hk_addr = write_file_parameters as *const () as lm_address_t;
    let close_file_params_hk_addr = close_file_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x413D14, get_registry_game_status_hk_addr).unwrap();
    let _ = LM_HookCode(0x4030D1, get_current_directory_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x402FC2, find_file_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x403070, get_directory_path_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x403105, set_current_directory_hk_addr).unwrap();
    let _ = LM_HookCode(0x402E3D, read_file_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x40301F, find_close_params_hk_addr).unwrap();
    let _ = LM_HookCode(
        0x412DA3,
        is_game_installed_in_current_directory_params_hk_addr,
    )
    .unwrap();
    let _ = LM_HookCode(0x402DE8, open_or_create_file_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x402E75, set_file_pointer_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x403206, build_file_pattern_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x402E57, write_file_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x402E23, close_file_params_hk_addr).unwrap();
}

#[naked]
unsafe extern "C" fn is_game_installed_in_current_directory_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("pusha", "call {}", "popa", "ret", sym is_game_installed_in_current_directory, options(noreturn));
}

unsafe fn is_game_installed_in_current_directory() {
    // In the original code here we'd check if the retrieved string and the registry string are equal
    // However, to have the game be portable, we'll skip this check
    // We still get the registry status and source keys though, as well as the current directory
    // As they are used in other functions
    get_registry_game_status();

    let game_path = 0x94E8B0 as *mut [u8; 0xFF];

    get_current_directory(&mut *game_path);

    string::are_strings_equal();
}

#[naked]
unsafe extern "C" fn get_current_directory_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push ebx", "push ecx", "push edx", "call {}", "pop edx", "pop ecx", "pop ebx", "ret", sym get_current_directory, options(noreturn));
}

// Gets the current directory
unsafe fn get_current_directory(directory_buffer: &mut [u8; 0xFF]) -> u32 {
    let bytes_written = GetCurrentDirectoryA(Some(directory_buffer));

    if bytes_written != 0 {
        // Convert current_directory to a String
        let mut current_directory_str =
            std::str::from_utf8_unchecked(&directory_buffer[..bytes_written as usize]).to_string();

        if !current_directory_str.ends_with('\\') && !current_directory_str.ends_with('/') {
            current_directory_str.push_str("\\\x00");

            directory_buffer[..current_directory_str.len()]
                .copy_from_slice(current_directory_str.as_bytes());
        }
    }

    bytes_written
}

// Gets the status and source keys from the registry
unsafe fn get_registry_game_status() {
    const SOFTWARE_MAGNETIC_FIELDS: &str = "SOFTWARE\\Magnetic Fields\\RC99\x00";
    const STATUS: &str = "status\x00";
    const SOURCE: &str = "source\x00";

    let mut lp_type: REG_VALUE_TYPE = REG_VALUE_TYPE::default();
    let mut registry_status = 0;
    let mut registry_source = vec![0u8; 0xFF];
    let mut lp_cb_data: u32;
    let mut phk_result: HKEY = HKEY::default();

    let mut result = RegOpenKeyExA(
        HKEY_LOCAL_MACHINE,
        PCSTR(SOFTWARE_MAGNETIC_FIELDS.as_ptr()),
        0,
        KEY_READ | KEY_WOW64_32KEY,
        &mut phk_result,
    );
    if result.is_err() {
        return;
    }

    lp_cb_data = 1;
    result = RegQueryValueExA(
        phk_result,
        PCSTR(STATUS.as_ptr()),
        None,
        Some(&mut lp_type),
        Some(&mut registry_status),
        Some(&mut lp_cb_data),
    );

    if result.is_ok() {
        if lp_type != REG_BINARY || lp_cb_data != 1 {
            registry_status = 0;
        }

        lp_cb_data = 0xFF;

        result = RegQueryValueExA(
            phk_result,
            PCSTR(SOURCE.as_ptr()),
            None,
            Some(&mut lp_type),
            Some(registry_source.as_mut_ptr()),
            Some(&mut lp_cb_data),
        );
        if result.is_ok() && lp_type != REG_SZ {
            registry_source.clear();
        }
    }

    let _ = RegCloseKey(phk_result);

    // Set 0x51BB8C to registry_status
    *(0x51BB8C as *mut u8) = registry_status;

    // Set 0x51BB8D to registry_source (and copy all data)
    (0x51BB8D as *mut u8).copy_from(registry_source.as_mut_ptr(), 0xFF);
}

#[naked]
unsafe extern "C" fn find_file_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    // ECX and EDX are also needed further on, so we have to save it
    asm!("push ebx", "push ecx", "push edx", "push eax", "call {}", "add esp, 4", "pop edx", "pop ecx", "pop ebx", "ret", sym find_file_impl, options(noreturn));
}

unsafe fn find_file_impl(a1: *const u8) -> u32 {
    // This seems to be where the current file name is stored
    *(0x4F52B0 as *mut *const u8) = a1;

    let lp_filename = PCSTR::from_raw(a1);

    let result: u32;

    // Create a pointer with type WIN32_FIND_DATAA that points to 0x4E05A8
    // This is the same as the type of the second argument of FindFirstFileA
    // Ideally this would be done with libmem, but it seems to be broken for now
    let find_file_data = 0x4E05A8 as *mut WIN32_FIND_DATAA;
    if let Ok(find_first_file_result) = FindFirstFileA(lp_filename, find_file_data) {
        *(0x4E05A4 as *mut HANDLE) = find_first_file_result;
        //H_FIND_FILE = find_first_file_result;

        if (*find_file_data).dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY.0 != 0 {
            result = find_next_file() as u32;
        } else {
            result = (*find_file_data).dwFileAttributes;
        }
    } else {
        *(0x4E05A4 as *mut HANDLE) = INVALID_HANDLE_VALUE;
        //H_FIND_FILE = INVALID_HANDLE_VALUE;
        result = 0;
    }

    result
}

unsafe fn find_next_file() -> i32 {
    let mut result: BOOL;
    let find_file_data = 0x4E05A8 as *mut WIN32_FIND_DATAA;
    let mut unk_value: u32;

    loop {
        if FindNextFileA(H_FIND_FILE, 0x4E05A8 as *mut WIN32_FIND_DATAA).is_err() {
            result = BOOL(0);
            break;
        }
        result = BOOL((*find_file_data).dwFileAttributes as i32);

        unk_value = *(0x4E06E6 as *const u32);

        let flags = (*find_file_data).dwFileAttributes & unk_value;

        if flags == 0 {
            break;
        }
    }

    result.0
}

#[naked]
unsafe extern "C" fn get_directory_path_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    asm!("push eax", "push ecx", "push ebx", "push edx", "call {}", "mov ebx, eax", "pop edx", "add esp, 4", "pop ecx", "pop eax", "ret", sym get_directory_path, options(noreturn));
}

unsafe fn get_directory_path(a1: *const c_char, a2: *mut u8) -> usize {
    // Convert a1 pointer to a &str
    let a1_str = CStr::from_ptr(a1).to_str();

    if let Ok(a1_str) = a1_str {
        // Copy a1 to a2 until the last occurence of a '\\', '/' or ':'
        let temp_str = a1_str.rsplit_once(|c| c == '\\' || c == '/' || c == ':');

        if let Some((temp_str, _)) = temp_str {
            a2.copy_from(temp_str.as_ptr(), temp_str.len() + 1); // + 1 for the null terminator

            // Move the end of string to buffer it was moved into as it'll be needed later
            return a2 as usize + temp_str.len() + 1;
        }
    }

    0
}

unsafe fn set_current_directory(_new_directory: PCSTR) {
    // Ignore this for now, we want to keep the current directory
    //SetCurrentDirectoryA(new_directory);
}

#[naked]
unsafe extern "C" fn read_file_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    // We must also restore ECX and EDX as they are needed further on
    asm!("push edx", "push ebx", "push eax", "push ecx", "call {}", "pop ecx", "add esp, 8", "cmp edx, 1", "pop edx", "ret", sym read_file, options(noreturn));
}

unsafe fn read_file(
    number_of_bytes_to_read: u32,
    h_file: HANDLE,
    file_buffer: *mut c_void,
) -> (u32, u32) {
    let mut number_of_bytes_read: u32 = 0;

    let result = ReadFile(
        h_file.0,
        file_buffer,
        number_of_bytes_to_read,
        &mut number_of_bytes_read as *mut u32,
        std::ptr::null_mut(),
    );

    (number_of_bytes_read, result as u32)
}

#[naked]
unsafe extern "C" fn write_file_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push ebx", "push ecx", "push edx", "push ebx", "push eax", "push ecx", "call {}", "add esp, 12", "cmp edx, 1", "pop edx", "pop ecx", "pop ebx", "ret", sym write_file, options(noreturn));
}

// TODO: This method should return a Result, not a u32
pub unsafe fn write_file(
    number_of_bytes_to_write: u32,
    file_handle: HANDLE,
    file_buffer: *const u8,
) -> (u32, u32) {
    let mut number_of_bytes_written: u32 = 0;

    let result = WriteFile(
        file_handle.0,
        file_buffer,
        number_of_bytes_to_write,
        &mut number_of_bytes_written as *mut u32,
        std::ptr::null_mut(),
    );

    (number_of_bytes_written, result as u32)
}

#[naked]
unsafe extern "C" fn find_close_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("pusha", "call {}", "popa", "ret", sym find_close, options(noreturn));
}

unsafe fn find_close() {
    let h_find_file = *(0x4E05A4 as *mut HANDLE);
    if h_find_file != INVALID_HANDLE_VALUE {
        let _ = FindClose(h_find_file);
        *(0x4E05A4 as *mut HANDLE) = INVALID_HANDLE_VALUE;
    }
}

#[naked]
unsafe extern "C" fn close_file_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push ebx", "push ecx", "push edx", "push eax", "call {}", "add esp, 4", "cmp eax, 1", "pop edx", "pop ecx", "pop ebx", "ret", sym close_file, options(noreturn));
}

pub unsafe fn close_file(file_handle: HANDLE) -> u32 {
    *(0x4F52B0 as *mut HANDLE) = HANDLE::default();

    let result = CloseHandle(file_handle);

    result.is_ok() as u32
}

#[naked]
unsafe extern "C" fn open_or_create_file_parameters() {
    asm!("push ebx", "push ecx", "push edx", "push ebx", "push eax", "call {}", "add esp, 8", "lea ebx, [eax + 1]", "cmp ebx, 1", "pop edx", "pop ecx", "pop ebx", "ret", sym open_or_create_file, options(noreturn));
}

pub unsafe fn open_or_create_file(a1: *mut u8, a2: u32) -> HANDLE {
    // TODO: Replace this by a global to current_file_pattern
    let current_file_pattern = build_file_pattern(a1);

    *(0x4F52B0 as *mut *mut u8) = current_file_pattern;

    let current_file_pattern_pcstr = PCSTR::from_raw(current_file_pattern);

    let dw_desired_access = *((0x4E06EA as *mut u32).add(4 * a2 as usize));
    let dw_share_mode = *((0x4E06F2 as *mut u32).add(4 * a2 as usize));
    let dw_creation_disposition = *((0x4E06EE as *mut u32).add(4 * a2 as usize));

    let file_handle = CreateFileA(
        current_file_pattern_pcstr,
        dw_desired_access,
        FILE_SHARE_MODE(dw_share_mode),
        None,
        FILE_CREATION_DISPOSITION(dw_creation_disposition),
        FILE_ATTRIBUTE_NORMAL,
        None,
    );

    if let Ok(file_handle) = file_handle {
        file_handle
    } else {
        INVALID_HANDLE_VALUE
    }
}

#[naked]
unsafe extern "C" fn set_file_pointer_parameters() {
    asm!("push ebx", "push ecx", "push edx", "push ebx", "push eax", "push ecx", "call {}", "add esp, 12", "lea edx, [eax + 1]", "cmp edx, 1", "pop edx", "pop ecx", "pop ebx", "ret", sym set_file_pointer, options(noreturn));
}

// TODO: Should return a Result, not a u32
pub unsafe fn set_file_pointer(distance_to_move: i32, file_handle: HANDLE, dw_move_method: SET_FILE_POINTER_MOVE_METHOD) -> u32 {
    SetFilePointer(
        file_handle,
        distance_to_move,
        None,
        dw_move_method,
    )
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

    if let (Ok(first_dword), Ok(first_word), Ok(first_char), Ok(second_char)) =
        (first_dword, first_word, first_char, second_char)
    {
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

                if current_char != 0x25
                /* '%' */
                {
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
