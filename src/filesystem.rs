use libmem::*;
use std::{
    arch::asm,
    ffi::{c_char, CStr},
};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HANDLE, INVALID_HANDLE_VALUE},
        Storage::FileSystem::{
            FindClose, FindFirstFileA, FindNextFileA, ReadFile, FILE_ATTRIBUTE_DIRECTORY,
            WIN32_FIND_DATAA,
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

static mut H_FIND_FILE: HANDLE = INVALID_HANDLE_VALUE;

pub fn inject_hooks() {
    let maybe_are_strings_equal_hk_addr = maybe_are_strings_equal as *const () as lm_address_t;
    let maybe_get_registry_game_status_hk_addr =
        maybe_get_registry_game_status as *const () as lm_address_t;
    let maybe_get_current_directory_params_hk_addr =
        maybe_get_current_directory_parameters as *const () as lm_address_t;
    let maybe_find_file_params_hk_addr = maybe_find_file_parameters as *const () as lm_address_t;
    let maybe_get_directory_path_params_hk_addr =
        maybe_get_directory_path_parameters as *const () as lm_address_t;
    let set_current_directory_hk_addr = set_current_directory as *const () as lm_address_t;
    let maybe_read_file_params_hk_addr = maybe_read_file_parameters as *const () as lm_address_t;
    let maybe_find_close_params_hk_addr = maybe_find_close_parameters as *const () as lm_address_t;
    let is_game_installed_in_current_directory_params_hk_addr =
        is_game_installed_in_current_directory_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x401EDE, maybe_are_strings_equal_hk_addr).unwrap();
    let _ = LM_HookCode(0x413D14, maybe_get_registry_game_status_hk_addr).unwrap();
    let _ = LM_HookCode(0x4030D1, maybe_get_current_directory_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x402FC2, maybe_find_file_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x403070, maybe_get_directory_path_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x403105, set_current_directory_hk_addr).unwrap();
    let _ = LM_HookCode(0x402E3D, maybe_read_file_params_hk_addr).unwrap();
    let _ = LM_HookCode(0x40301F, maybe_find_close_params_hk_addr).unwrap();
    let _ = LM_HookCode(
        0x412DA3,
        is_game_installed_in_current_directory_params_hk_addr,
    )
    .unwrap();
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
    maybe_get_registry_game_status();

    let game_path = 0x94E8B0 as *mut [u8; 0xFF];

    maybe_get_current_directory(&mut *game_path);

    maybe_are_strings_equal();
}

// This method is used to check if the game is installed to the correct folder (among possibly other things)
// It sets ZF to 1 if the game is installed to the correct folder, and 0 otherwise
// As such, force ZF to 1 to skip this check
unsafe fn maybe_are_strings_equal() {
    asm!("xor eax, eax",);
}

#[naked]
unsafe extern "C" fn maybe_get_current_directory_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("push ebx", "push ecx", "push edx", "call {}", "pop edx", "pop ecx", "pop ebx", "ret", sym maybe_get_current_directory, options(noreturn));
}

// Gets the current directory
unsafe fn maybe_get_current_directory(directory_buffer: &mut [u8; 0xFF]) -> u32 {
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
unsafe fn maybe_get_registry_game_status() {
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
unsafe extern "C" fn maybe_find_file_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    // ECX and EDX are also needed further on, so we have to save it
    asm!("push ebx", "push ecx", "push edx", "push eax", "call {}", "add esp, 4", "pop edx", "pop ecx", "pop ebx", "ret", sym maybe_find_file_impl, options(noreturn));
}

unsafe fn maybe_find_file_impl(a1: *const u8) -> u32 {
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
            result = maybe_find_next_file() as u32;
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

unsafe fn maybe_find_next_file() -> i32 {
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
unsafe extern "C" fn maybe_get_directory_path_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    asm!("push eax", "push ecx", "push ebx", "push edx", "call {}", "mov ebx, eax", "pop edx", "add esp, 4", "pop ecx", "pop eax", "ret", sym maybe_get_directory_path, options(noreturn));
}

unsafe fn maybe_get_directory_path(a1: *const c_char, a2: *mut u8) -> usize {
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
unsafe extern "C" fn maybe_read_file_parameters() {
    // Push the parameters to the stack
    // As the parameters are not passed as normal, and Rust can't handle it, we have to do it manually
    // We must also restore ECX and EDX as they are needed further on
    asm!("push edx", "push ebx", "push eax", "push ecx", "call {}", "pop ecx", "add esp, 8", "pop edx", "ret", sym maybe_read_file, options(noreturn));
}

// TODO: Figure out why the map doesn't load properly
// This seems to be the culprit
unsafe fn maybe_read_file(
    number_of_bytes_to_read: *mut u32,
    h_file: HANDLE,
    file_buffer: &mut [u8],
) -> u32 {
    let mut number_of_bytes_read: u32 = 0;
    let mut temp_buffer = vec![0u8; number_of_bytes_to_read as usize];

    if ReadFile(
        h_file,
        Some(&mut temp_buffer),
        Some(&mut number_of_bytes_read as *mut u32),
        None,
    )
    .is_ok()
    {
        // Copy byte by byte to the original buffer using the pointer
        // There are probably better ways to do this, but I can't find one to copy a vector to an array
        (0..number_of_bytes_read as usize).for_each(|i| {
            *(file_buffer.as_mut_ptr().add(i)) = temp_buffer[i];
        });
    }

    number_of_bytes_read
}

#[naked]
unsafe extern "C" fn maybe_find_close_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("pusha", "call {}", "popa", "ret", sym maybe_find_close, options(noreturn));
}

unsafe fn maybe_find_close() {
    let h_find_file = *(0x4E05A4 as *mut HANDLE);
    if h_find_file != INVALID_HANDLE_VALUE {
        let _ = FindClose(h_find_file);
        *(0x4E05A4 as *mut HANDLE) = INVALID_HANDLE_VALUE;
    }
}
