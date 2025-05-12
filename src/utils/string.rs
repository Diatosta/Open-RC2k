use libmem::{hook_code, Address};
use std::arch::{asm, naked_asm};

pub unsafe fn inject_hooks() { unsafe {
    let are_strings_equal_hk_addr = are_strings_equal as *const () as Address;
    let insert_new_line_params_hk_addr = insert_new_line_parameters as *const () as Address;
    let insert_space_params_hk_addr = insert_space_parameters as *const () as Address;
    let insert_tab_params_hk_addr = insert_tab_parameters as *const () as Address;
    let insert_colon_params_hk_addr = insert_colon_parameters as *const () as Address;
    let insert_dash_params_hk_addr = insert_dash_parameters as *const () as Address;
    let append_params_hk_addr = append_parameters as *const () as Address;

    let _ = hook_code(0x401EDE, are_strings_equal_hk_addr).unwrap();
    let _ = hook_code(0x401F20, insert_new_line_params_hk_addr).unwrap();
    let _ = hook_code(0x401F29, insert_space_params_hk_addr).unwrap();
    let _ = hook_code(0x401F2E, insert_tab_params_hk_addr).unwrap();
    let _ = hook_code(0x401F33, insert_colon_params_hk_addr).unwrap();
    let _ = hook_code(0x401F4C, insert_dash_params_hk_addr).unwrap();
    let _ = hook_code(0x401F7D, append_params_hk_addr).unwrap();
}}

#[unsafe(naked)]
unsafe extern "C" fn append_parameters() {
    naked_asm!("push ecx", "push edx", "push eax", "push ebx", "call {}", "mov ebx, eax", "add esp, 4", "pop eax", "pop edx", "pop ecx", "ret", sym append_hooked);
}

pub unsafe fn append_hooked(mut destination: *mut u8, mut source: *const u8) -> *mut u8 { unsafe {
    // TODO: Replace this by a more idiomatic way
    // For now we'll have to deal with raw pointers
    loop {
        let current_char = *source;

        if current_char == 0 {
            break;
        }

        *destination = current_char;

        source = source.add(1);
        destination = destination.add(1);
    }

    destination
}}

pub unsafe fn append(destination: &mut String, source: &str) {
    destination.push_str(source);
}

#[unsafe(naked)]
unsafe extern "C" fn insert_new_line_parameters() {
    naked_asm!("push ebx", "call {}", "add esp, 4", "ret", sym insert_new_line_hooked);
}

// This method supposes the passed pointer points to the end of the string
// TODO: Replace this by method below when all calls to it are replaced
unsafe fn insert_new_line_hooked(string: *mut u16) { unsafe {
    *string = 0xA0D; // \n\r

    // Move pointer 2 forward so we don't overwrite later
    asm!("add ebx, 2");
}}

pub unsafe fn insert_new_line(string: &mut String) {
    string.push_str("\r\n");
}

#[unsafe(naked)]
unsafe extern "C" fn insert_space_parameters() {
    naked_asm!("push ebx", "call {}", "add esp, 4", "ret", sym insert_space_hooked);
}

// This method supposes the passed pointer points to the end of the string
// TODO: Replace this by method below when all calls to it are replaced
unsafe fn insert_space_hooked(string: *mut u8) { unsafe {
    *string = 0x20; // Space

    // Move pointer 1 forward so we don't overwrite later
    asm!("add ebx, 1");
}}

pub unsafe fn insert_space(string: &mut String) {
    string.push(' ');
}

pub unsafe fn insert_null_terminator(string: &mut String) {
    string.push('\0');
}

#[unsafe(naked)]
unsafe extern "C" fn insert_tab_parameters() {
    naked_asm!("push ebx", "call {}", "add esp, 4", "ret", sym insert_tab);
}

// This method supposes the passed pointer points to the end of the string
unsafe fn insert_tab(string: *mut u8) { unsafe {
    *string = 0x9; // Tab

    // Move pointer 1 forward so we don't overwrite later
    asm!("add ebx, 1");
}}

#[unsafe(naked)]
unsafe extern "C" fn insert_colon_parameters() {
    naked_asm!("push ebx", "call {}", "add esp, 4", "ret", sym insert_colon);
}

// This method supposes the passed pointer points to the end of the string
unsafe fn insert_colon(string: *mut u8) { unsafe {
    *string = 0x3A; // :

    // Move pointer 1 forward so we don't overwrite later
    asm!("add ebx, 1");
}}

#[unsafe(naked)]
unsafe extern "C" fn insert_dash_parameters() {
    naked_asm!("push ebx", "call {}", "add esp, 4", "ret", sym insert_dash_hooked);
}

// This method supposes the passed pointer points to the end of the string
pub unsafe fn insert_dash_hooked(string: *mut u8) { unsafe {
    *string = b'-';

    // Move pointer 1 forward so we don't overwrite later
    asm!("add ebx, 1");
}}

pub unsafe fn insert_dash(string: &mut String) {
    string.push('-');
}

// This method is used to check if the game is installed to the correct folder (among possibly other things)
// It sets ZF to 1 if the game is installed to the correct folder, and 0 otherwise
// As such, force ZF to 1 to skip this check
pub unsafe fn are_strings_equal() { unsafe {
    asm!("xor eax, eax",);
}}
