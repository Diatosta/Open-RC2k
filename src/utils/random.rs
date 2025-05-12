use crate::utils::datetime::CURRENT_SYSTEM_TIME;
use libmem::{hook_code, Address};
use std::arch::{asm, naked_asm};

pub unsafe fn inject_hooks() { unsafe {
    let rng_with_seeds_hk_addr = rng_with_seeds_parameters as *const () as Address;
    let update_rng_seed_current_system_time_hk_addr =
        update_rng_seed_current_system_time_parameters as *const () as Address;

    let _ = hook_code(0x4018C8, rng_with_seeds_hk_addr).unwrap();
    let _ = hook_code(0x401864, update_rng_seed_current_system_time_hk_addr).unwrap();
}}

#[unsafe(naked)]
unsafe extern "C" fn rng_with_seeds_parameters() {
    naked_asm!("push edx", "push ecx", "push ebx", "push eax", "call {}", "add esp, 8", "pop ecx", "pop edx", "ret", sym rng_with_seeds_hooked);
}

pub unsafe fn rng_with_seeds_hooked(seed1: u32, seed2: u32) -> u32 {
    // TODO: Replace by a more idiomatic way
    let mut new_rand = (((((((seed1 & 0x400000) != 0) as u32) ^ seed1) & 4) != 0) as u32)
        ^ (((seed1 & 0x400000) != 0) as u32)
        ^ seed1;
    new_rand = (((new_rand & 2) != 0) as u32) ^ new_rand;
    seed2 ^ u32::rotate_right(new_rand, 1)
}

pub fn rng_with_seeds(seed1: u32, seed2: u32) -> u32 {
    let mut new_rand = seed1;
    new_rand ^= new_rand & 0x400000;
    new_rand ^= new_rand & 4;
    new_rand ^= new_rand & 2;

    seed2 ^ (new_rand.rotate_right(1))
}

#[unsafe(naked)]
unsafe extern "C" fn update_rng_seed_current_system_time_parameters() {
    naked_asm!("pusha", "call {}", "popa", "ret", sym update_rng_seed_current_system_time_hooked);
}

pub unsafe fn update_rng_seed_current_system_time_hooked() { unsafe {
    let properties = match CURRENT_SYSTEM_TIME.try_lock() {
        Ok(properties) => properties,
        Err(e) => {
            println!("Failed to lock CURRENT_SYSTEM_TIME: {}", e);
            return;
        }
    };

    let result = rng_with_seeds_hooked(properties.hour as u32, properties.day as u32);
    if result != 0 {
        // TODO: replace by reference to random_number when all references to it are replaced
        *(0x4E0140 as *mut u32) = u32::rotate_left(result, properties.day_of_week as u32 + 8);
    }
}}
