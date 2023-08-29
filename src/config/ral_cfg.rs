use std::{sync::{LazyLock, Mutex}, arch::asm};

use libmem::{lm_address_t, LM_HookCode};

use crate::filesystem;

// TODO: This should be declared in main and passed around
pub static RAL_CFG_PROPERTIES: LazyLock<Mutex<RalCfgProperties>> = LazyLock::new(|| Mutex::new(RalCfgProperties::new()));

#[derive(Default, Debug)]
pub struct RalCfgProperties {
    pub no_task_switch: i32,
    pub no_pri_change: i32,
    pub ignore_mmx: i32,
    pub ignore_katmai: i32,
    pub ignore_amd_k3d: i32,
    pub alt_mem_enable: i32,
    pub alt_mem_heap_size: i32,
    pub no_audio: i32,
    pub no_sound: i32,
    pub no_cd_audio: i32,
    pub no_music: i32,
    pub audio_base_freq: i32,
    pub ignore_ds3d: i32,
    pub ignore_dsv: i32,
    pub ignore_eax: i32,
    pub ignore_a3d: i32,
    pub wav_heap_size: i32,
    pub bundle_heap_size: i32,
    pub image_heap_size: i32,
    pub game_gfx_heap_size: i32,
    pub force_dx5: i32,
    pub force_no3d_card: i32,
    pub ignore_primary: i32,
    pub ignore_secondary: i32,
    pub agp_mode: i32,
    pub force3d_trans: i32,
    pub force3d_alpha: i32,
    pub force3d_shadow: i32,
    pub force3d_fogging: i32,
    pub text_flash_mode: i32,
    pub video_mode_main: i32,
    pub video_mode_game: i32,
    pub v3_driver_fix: i32,
    pub no_png_decode: i32,
    pub no_joy: i32,
    pub joy_mode: i32,
    pub joy_code: i32,
    pub joy_lower_dz: i32,
    pub joy_upper_dz: i32,
    pub no_force_feedback: i32,
    pub joy_ff_gain: i32,
    pub merge_joy: i32,
    pub no_network: i32,
    pub inet_port: i32,
    pub screen_snapshot: i32,
    pub frame_counter: i32,
    pub no_replay: i32,
    pub replay_keys: i32,
    pub log_file: i32,
    pub no_fmv: i32,
    pub key: i32,
    pub all_secret_cars: i32,
    pub secret_car: i32,
    pub use_savegame_keys: i32,
    pub sensible_saving: i32,
    pub no_precipitation: i32,
    pub season_code: i32,
    pub no_auto_neutral: i32,
    pub non_linear_steering: i32,
    pub wr_replay_enable: i32,
    pub wr_replay_rate: i32,
    pub alt2_ax_code: i32,
    pub fiz_patch: i32,
    pub patch_max_draw_distance: i32,
    pub patch_sticky_grass: i32,
    pub patch_gravity: i32,
    pub patch_damages: i32,
    pub patch_brake_grip: i32,
    pub patch_accel_grip: i32,
    pub patch_slide_grip: i32,
    pub patch_grip_grass: i32,
    pub patch_grip_tarmac: i32,
    pub patch_grip_snow: i32,
    pub patch_grip_mud: i32,
    pub patch_grip_gravel: i32,
    pub lex_control: i32,
    pub sparks_mode: i32,
}

impl RalCfgProperties {
    fn new() -> Self {
        Default::default()
    }

    pub fn set_property(&mut self, key: &str, value: i32) {
        match key {
            "notaskswitch" => self.no_task_switch = value,
            "noprichange" => self.no_pri_change = value,
            "ignoremmx" => self.ignore_mmx = value,
            "ignorekatmai" => self.ignore_katmai = value,
            "ignoreamd3d" => self.ignore_amd_k3d = value,
            "altmemenable" => self.alt_mem_enable = value,
            "altmemheapsize" => self.alt_mem_heap_size = value,
            "noaudio" => self.no_audio = value,
            "nosound" => self.no_sound = value,
            "nocdaudio" => self.no_cd_audio = value,
            "nomusic" => self.no_music = value,
            "audiobasefreq" => self.audio_base_freq = value,
            "ignoreds3d" => self.ignore_ds3d = value,
            "ignoredsv" => self.ignore_dsv = value,
            "ignoreeax" => self.ignore_eax = value,
            "ignorea3d" => self.ignore_a3d = value,
            "wavheapsize" => self.wav_heap_size = value,
            "bundleheapsize" => self.bundle_heap_size = value,
            "imageheapsize" => self.image_heap_size = value,
            "gamegfxheapsize" => self.game_gfx_heap_size = value,
            "forcedx5" => self.force_dx5 = value,
            "forceno3dcard" => self.force_no3d_card = value,
            "ignoreprimary" => self.ignore_primary = value,
            "ignoresecondary" => self.ignore_secondary = value,
            "agpmode" => self.agp_mode = value,
            "force3dtrans" => self.force3d_trans = value,
            "force3dalpha" => self.force3d_alpha = value,
            "force3dshadow" => self.force3d_shadow = value,
            "force3dfogging" => self.force3d_fogging = value,
            "textflashmode" => self.text_flash_mode = value,
            "videomodemain" => self.video_mode_main = value,
            "videomodegame" => self.video_mode_game = value,
            "v3driverfix" => self.v3_driver_fix = value,
            "nopngdecode" => self.no_png_decode = value,
            "nojoy" => self.no_joy = value,
            "joymode" => self.joy_mode = value,
            "joycode" => self.joy_code = value,
            "joylowerdz" => self.joy_lower_dz = value,
            "joyupperdz" => self.joy_upper_dz = value,
            "noforcefeedback" => self.no_force_feedback = value,
            "joyffgain" => self.joy_ff_gain = value,
            "mergejoy" => self.merge_joy = value,
            "nonetwork" => self.no_network = value,
            "inetport" => self.inet_port = value,
            "screensnapshot" => self.screen_snapshot = value,
            "framecounter" => self.frame_counter = value,
            "noreplay" => self.no_replay = value,
            "replaykeys" => self.replay_keys = value,
            "logfile" => self.log_file = value,
            "nofmv" => self.no_fmv = value,
            "key" => self.key = value,
            "allsecretcars" => self.all_secret_cars = value,
            "secretcar" => self.secret_car = value,
            "usesavegamekeys" => self.use_savegame_keys = value,
            "sensiblesaving" => self.sensible_saving = value,
            "noprecipitation" => self.no_precipitation = value,
            "seasoncode" => self.season_code = value,
            "noautoneutral" => self.no_auto_neutral = value,
            "nonlinearsteering" => self.non_linear_steering = value,
            "wrreplayenable" => self.wr_replay_enable = value,
            "wrreplayrate" => self.wr_replay_rate = value,
            "alt2axcode" => self.alt2_ax_code = value,
            "fizpatch" => self.fiz_patch = value,
            "patchmaxdrawdistance" => self.patch_max_draw_distance = value,
            "patchstickygrass" => self.patch_sticky_grass = value,
            "patchgravity" => self.patch_gravity = value,
            "patchdamages" => self.patch_damages = value,
            "patchbrakegrip" => self.patch_brake_grip = value,
            "patchaccelgrip" => self.patch_accel_grip = value,
            "patchslidegrip" => self.patch_slide_grip = value,
            "patchgripgrass" => self.patch_grip_grass = value,
            "patchgriptarmac" => self.patch_grip_tarmac = value,
            "patchgripsnow" => self.patch_grip_snow = value,
            "patchgripmud" => self.patch_grip_mud = value,
            "patchgripgravel" => self.patch_grip_gravel = value,
            "lexcontrol" => self.lex_control = value,
            "sparksmode" => self.sparks_mode = value,
            _ => {}
        }
    }
    
    pub fn get_address(key: &str) -> Result<usize, ()> {
        match key {
            "notaskswitch" => Ok(0x518384),
            "noprichange" => Ok(0x518388),
            "ignoremmx" => Ok(0x5183AC),
            "ignorekatmai" => Ok(0x5183B4),
            "ignoreamdk3d" => Ok(0x5183BC),
            "altmemenable" => Ok(0x4E0584),
            "altmemheapsize" => Ok(0x4E0588),
            "noaudio" => Ok(0x4F82B4),
            "nosound" => Ok(0x4F82B4),
            "nocdaudio" => Ok(0x518104),
            "nomusic" => Ok(0x5069BC),
            "audiobasefreq" => Ok(0x4F82D4),
            "ignoreds3d" => Ok(0x506B42),
            "ignoredsv" => Ok(0x506C50),
            "ignoreeax" => Ok(0x506C6C),
            "ignorea3d" => Ok(0x506C5C),
            "wavheapsize" => Ok(0x5068EC),
            "bundleheapsize" => Ok(0x51B980),
            "imageheapsize" => Ok(0x519554),
            "gamegfxheapsize" => Ok(0x5BE894),
            "forcedx5" => Ok(0x5AB958),
            "forceno3dcard" => Ok(0x5AB94C),
            "ignoreprimary" => Ok(0x5AB950),
            "ignoresecondary" => Ok(0x5AB954),
            "agpmode" => Ok(0x5AB95C),
            "force3dtrans" => Ok(0x5AB97C),
            "force3dalpha" => Ok(0x5AB984),
            "force3dshadow" => Ok(0x5AB980),
            "force3dfogging" => Ok(0x5AB988),
            "textflashmode" => Ok(0x4F3F74),
            "videomodemain" => Ok(0x4F4944),
            "videomodegame" => Ok(0x4F4948),
            "v3driverfix" => Ok(0x624238),
            "nopngdecode" => Ok(0x51B97C),
            "nojoy" => Ok(0x4F68DC),
            "joymode" => Ok(0x4F68E0),
            "joycode" => Ok(0x4F68E4),
            "joylowerdz" => Ok(0x4F68E8),
            "joyupperdz" => Ok(0x4F68EC),
            "noforcefeedback" => Ok(0x4F68F4),
            "joyffgain" => Ok(0x4F68F8),
            "mergejoy" => Ok(0x4F68FC),
            "nonetwork" => Ok(0x52A4B0),
            "inetport" => Ok(0x52AC24),
            "screensnapshot" => Ok(0x4F52B4),
            "framecounter" => Ok(0x5EB100),
            "noreplay" => Ok(0x5BEC7C),
            "replaykeys" => Ok(0x5BECA8),
            "logfile" => Ok(0x4E0098),
            "nofmv" => Ok(0x5BE870),
            "key" => Ok(0x4E052C),
            "allsecretcars" => Ok(0x758710),
            "secretcar" => Ok(0x758710),
            "usesavegamekeys" => Ok(0x755144),
            "sensiblesaving" => Ok(0x75AAD8),
            "noprecipitation" => Ok(0x71C800),
            "seasoncode" => Ok(0x5BECAC),
            "noautoneutral" => Ok(0x61416C),
            "nonlinearsteering" => Ok(0x4F6900),
            "wrreplayenable" => Ok(0x5C2CEB),
            "wrreplayrate" => Ok(0x5C2CEF),
            "alt2axcode" => Ok(0x4F68F0),
            "fizpatch" => Ok(0x71C6D0),
            "patchmaxdrawdistance" => Ok(0x71C6D4),
            "patchstickygrass" => Ok(0x71C6D8),
            "patchgravity" => Ok(0x71C6DC),
            "patchdamages" => Ok(0x71C6E0),
            "patchbrakegrip" => Ok(0x71C6E4),
            "patchaccelgrip" => Ok(0x71C6E8),
            "patchslidegrip" => Ok(0x71C6EC),
            "patchgripgrass" => Ok(0x71C6F0),
            "patchgriptarmac" => Ok(0x71C6F4),
            "patchgripsnow" => Ok(0x71C6F8),
            "patchgripmud" => Ok(0x71C6FC),
            "patchgripgravel" => Ok(0x71C700),
            "lexcontrol" => Ok(0x4E0758),
            "sparksmode" => Ok(0x7558DC),
            _ => Err(()),
        }
    }
}

pub fn inject_hooks() {
    let load_ral_cfg_entries_parameters_hk_addr = load_ral_cfg_entries_parameters as *const () as lm_address_t;

    let _ = LM_HookCode(0x4114A6, load_ral_cfg_entries_parameters_hk_addr).unwrap();
}

unsafe fn parse_ral_cfg_entries(file_buffer: &str) {
    let file_lines = file_buffer.lines();

    let mut properties = match RAL_CFG_PROPERTIES.try_lock() {
        Ok(properties) => properties,
        Err(e) => {
            println!("Failed to lock RAL_CFG_PROPERTIES: {}", e);
            return;
        }
    };

    for line in file_lines {
        let (key, value) = match line.trim().split_once('=') {
            Some((key, value)) => (key, value),
            None => {
                println!("Invalid line in RAL.CFG: {}", line);
                continue;
            }
        };

        let value_parsed = match value.parse::<i32>() {
            Ok(value_parsed) => value_parsed,
            Err(_) => {
                // Try to parse the value as a hexadecimal number
                let value_without_prefix = value.trim_start_matches("0x");
                match i32::from_str_radix(value_without_prefix, 16) {
                    Ok(value_parsed) => value_parsed,
                    Err(_) => {
                        println!("Invalid value in RAL.CFG: {}", line);
                        continue;
                    }
                }
            }
        };

        let property_address = match RalCfgProperties::get_address(key) {
            Ok(property_address) => property_address,
            Err(_) => {
                println!("Invalid key in RAL.CFG: {}", line);
                continue;
            }
        };

        *(property_address as *mut i32) = value_parsed;

        properties.set_property(key, value_parsed);
    }

    println!("Loaded RAL.CFG properties: {:?}", properties);
}

#[naked]
unsafe extern "C" fn load_ral_cfg_entries_parameters() {
    asm!("pusha", "call {}", "popa", "ret", sym load_ral_cfg_hooked, options(noreturn));
}

unsafe fn load_ral_cfg_hooked() {
    let ral_cfg_file_name = "var\\ral.cfg";

    let ral_cfg_file = match filesystem::load_file_plaintext(ral_cfg_file_name) {
        Ok(ral_cfg_file) => ral_cfg_file,
        Err(e) => {
            println!("Failed to load RAL.CFG: {}", e);
            return;
        }
    };

    parse_ral_cfg_entries(&ral_cfg_file)
}