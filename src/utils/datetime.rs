use libmem::*;
use std::{
    arch::asm,
    sync::{LazyLock, Mutex},
};
use windows::Win32::System::SystemInformation::GetLocalTime;

// TODO: This should be declared in main and passed around
pub static CURRENT_SYSTEM_TIME: LazyLock<Mutex<CurrentSystemTime>> =
    LazyLock::new(|| Mutex::new(CurrentSystemTime::new()));

#[derive(Default, Debug)]
pub struct CurrentSystemTime {
    day_of_week: u16,
    day: u16,
    month: u16,
    year: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

impl CurrentSystemTime {
    fn new() -> Self {
        Self {
            day_of_week: 0,
            day: 0,
            month: 0,
            year: 0,
            hour: 0,
            minute: 0,
            second: 0,
            milliseconds: 0,
        }
    }
}

pub fn inject_hooks() {
    let get_current_system_time_params_hk_addr =
        get_current_system_time_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x410F29, get_current_system_time_params_hk_addr).unwrap();
}

#[naked]
unsafe extern "C" fn get_current_system_time_parameters() {
    // We must push and pop all registers as they are needed further on
    asm!("pusha", "call {}", "popa", "ret", sym get_current_system_time_hooked, options(noreturn));
}

pub fn get_current_system_time_hooked() {
    let mut properties = match CURRENT_SYSTEM_TIME.try_lock() {
        Ok(properties) => properties,
        Err(e) => {
            println!("Failed to lock CURRENT_SYSTEM_TIME: {}", e);
            return;
        }
    };

    unsafe {
        let local_time = GetLocalTime();

        properties.day_of_week = local_time.wDayOfWeek;
        properties.day = local_time.wDay;
        properties.month = local_time.wMonth;
        properties.year = local_time.wYear;
        properties.hour = local_time.wHour;
        properties.minute = local_time.wMinute;
        properties.second = local_time.wSecond;
        properties.milliseconds = local_time.wMilliseconds / 0xA;

        *(0x51839F as *mut u8) = local_time.wDayOfWeek as u8;
        *(0x5183A0 as *mut u8) = local_time.wDay as u8;
        *(0x5183A1 as *mut u8) = local_time.wMonth as u8;
        *(0x5183A3 as *mut u8) = local_time.wYear as u8;
        *(0x5183A4 as *mut u8) = local_time.wHour as u8;
        *(0x5183A5 as *mut u8) = local_time.wMinute as u8;
        *(0x5183A6 as *mut u8) = local_time.wSecond as u8;
        *(0x5183A7 as *mut u8) = (local_time.wMilliseconds / 0xA) as u8;
    }
}

pub fn get_current_system_time() -> CurrentSystemTime {
    unsafe {
        let local_time = GetLocalTime();

        CurrentSystemTime {
            day_of_week: local_time.wDayOfWeek,
            day: local_time.wDay,
            month: local_time.wMonth,
            year: local_time.wYear,
            hour: local_time.wHour,
            minute: local_time.wMinute,
            second: local_time.wSecond,
            milliseconds: local_time.wMilliseconds / 0xA,
        }
    }
}
