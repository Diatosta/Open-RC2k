use libmem::*;
use std::arch::asm;

pub fn inject_hooks() {
    let parse_int_params_hk_addr = parse_int_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x411623, parse_int_params_hk_addr).unwrap();
}

#[naked]
unsafe extern "C" fn parse_int_parameters() {
    asm!("push ebx", "push eax", "call {}", "add esp, 4", "pop ebx", "ret", sym parse_int_hooked, options(noreturn));
}

pub unsafe fn parse_int_hooked(string: *mut u8) -> i32 {
    let mut length = 0;
    let mut offset = 0;
    let mut current_char: u8;
    let mut char_vec = Vec::new();
    let mut negate_result = false;
    let mut hexadecimal = false;

    if *string == b'+' {
        offset = 1;
    }

    if *string == b'-' {
        offset = 1;
        negate_result = true;
    }

    if *string == b'$' {
        offset = 1;
        hexadecimal = true;
    }

    if &[*string, *string.add(1)] == b"0x" {
        offset = 2;
        hexadecimal = true;
    }

    current_char = *string.offset(offset);

    // Find how many characters are in the string until the first non-digit character
    while (48..=57).contains(&current_char) {
        char_vec.push(current_char);
        length += 1;
        current_char = *string.offset(length + offset);
    }

    // If the string is empty, return 0
    if length == 0 {
        return 0;
    }

    let mut result = 0;

    // Otherwise, parse the string
    if hexadecimal {
        let mut multiplier = 1;

        for i in (0..length).rev() {
            let current_char = char_vec[i as usize];

            if (48..=57).contains(&current_char) {
                result += (current_char - 48) as i32 * multiplier;
            } else if (65..=70).contains(&current_char) {
                result += (current_char - 55) as i32 * multiplier;
            } else if (97..=102).contains(&current_char) {
                result += (current_char - 87) as i32 * multiplier;
            }

            multiplier *= 16;
        }

        if negate_result {
            result *= -1;
        }
    } else {
        let mut multiplier = 1;

        for i in (0..length).rev() {
            result += (char_vec[i as usize] - 48) as i32 * multiplier;
            multiplier *= 10;
        }

        if negate_result {
            result *= -1;
        }
    }

    result
}
