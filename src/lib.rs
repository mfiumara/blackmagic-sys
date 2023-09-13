#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr;

// use {bmda_cli_mode_BMP_MODE_DEBUG, bmda_cli_options, bmp_scan_mode_BMP_SCAN_SWD, serial_open};

// Define your custom error type (Err)
#[derive(Debug)]
pub struct BlackMagicProbeError {
    message: String,
}

impl fmt::Display for BlackMagicProbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BlackMagicProbeError {}
/// A result type with the error hardwired to [`BmpError`].
pub type Result<T> = std::result::Result<T, BlackMagicProbeError>;

#[derive(Debug)]
pub struct BlackMagicProbe {
    // The serial number of the probe
    serial: String,
}

const MAX_SPEED: u32 = 4_000_000;

impl BlackMagicProbe {
    // Opens the device handle for the specified serial number of the Black Magic Probe
    pub fn open_by_serial(serial: &str) -> Result<Self> {
        // Set the probe type to BMDA, this normally gets set during platform_init which we do not want to call
        unsafe { bmda_probe_info.type_ = probe_type_PROBE_TYPE_BMP }
        let input = CString::new(serial).expect("CString conversion failed");
        let options = bmda_cli_options {
            opt_target_dev: 1,
            opt_flash_size: 0xffffffff,
            opt_flash_start: 0xffffffff,
            opt_max_swj_frequency: MAX_SPEED,
            opt_scanmode: bmp_scan_mode_BMP_SCAN_SWD,
            opt_mode: bmda_cli_mode_BMP_MODE_DEBUG,

            opt_tpwr: false,
            opt_list_only: false,
            opt_connect_under_reset: false,
            external_resistor_swd: false,
            fast_poll: false,
            opt_no_hl: false,
            opt_flash_file: ptr::null_mut(),
            opt_device: ptr::null_mut(),
            opt_serial: ptr::null_mut(),
            opt_targetid: 0,
            opt_ident_string: ptr::null_mut(),
            opt_position: 0,
            opt_cable: ptr::null_mut(),
            opt_monitor: ptr::null_mut(),
        };
        let result = unsafe { serial_open(&options, input.as_ptr()) };
        if result == false {
            return Err(BlackMagicProbeError {
                message: "Could not open serial device".to_string(),
            });
        }
        let bmp = BlackMagicProbe {
            serial: serial.to_string(),
        };

        let result = unsafe { remote_init(true) };
        if result == false {
            return Err(BlackMagicProbeError {
                message: "Could not initialize device".to_string(),
            });
        }

        let ret: Result<BlackMagicProbe> = Ok(bmp);
        return ret;
    }

    // Asserts the nrst line when input is true
    // Returns true or exits in case the call failed
    pub fn nrst_set(&self, assert: bool) -> Result<bool> {
        unsafe { remote_nrst_set_val(assert) }
        Ok(true)
    }

    // Get the maximum speed of the probe
    pub fn max_speed_get(&self) -> u32 {
        unsafe { platform_max_frequency_get() }
    }

    // Sets the maximum speed of the probe
    pub fn max_speed_set(&self, speed: u32) {
        unsafe { platform_max_frequency_set(speed) }
    }

    pub fn set_power(&self, enable: bool) -> Result<bool> {
        if unsafe { platform_target_set_power(enable) } {
            return Ok(enable);
        }
        Err(BlackMagicProbeError {
            message: "Could not set target power".to_string(),
        })
    }

    // Gets the target voltage
    pub fn target_voltage(&self) -> String {
        let cstr = unsafe { platform_target_voltage() };
        let c_str = unsafe { CStr::from_ptr(cstr) };

        // Attempt to convert the CStr to a &str (Rust string slice)
        match c_str.to_str() {
            Ok(rust_str) => {
                // The conversion succeeded
                return rust_str.to_string();
            }
            Err(_) => {
                // Handle the error (invalid UTF-8)
                println!("Invalid UTF-8 data in C string");
            }
        }
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    // Import necessary modules and functions
    #[test]
    fn test_remote_hex_string_to_num() {
        use super::remote_hex_string_to_num;
        use std::ffi::CString; // Adjust the import path as needed
        let input = CString::new("").expect("CString conversion failed");
        let result = unsafe { remote_hex_string_to_num(0, input.as_ptr()) };
        assert_eq!(result, 0);
        let input = CString::new("1").expect("CString conversion failed");
        let result = unsafe { remote_hex_string_to_num(1, input.as_ptr()) };
        assert_eq!(result, 1);
        let input = CString::new("123456789ABCDEF").expect("CString conversion failed");
        let result = unsafe { remote_hex_string_to_num(15, input.as_ptr()) };
        assert_eq!(result, 0x123456789ABCDEF);
    }

    #[test]
    // These tests are to be run with a bmp connected with the serial provided
    // In addition, a target needs to be connected with the specified supply voltage
    // A bmp connected to an nrf DK or thingy would be a proper set-up for
    // having the tests pass
    fn test_bmp_hil() {
        use std::thread::sleep;
        use std::time::Duration;
        let duration = Duration::from_millis(10);
        let serial = "98B72495";
        let target_voltage = "1.8V";

        // This test only works when a bmp is connect with the serial number specified in the variable below
        let bmp = super::BlackMagicProbe::open_by_serial(serial).unwrap();

        // Target voltage should be ON after initializing
        sleep(duration);
        assert_eq!(bmp.target_voltage(), target_voltage);

        // We should be able to set the speed to a lower value of the max
        let speed1 = bmp.max_speed_get();
        assert!(speed1 > 0);
        bmp.max_speed_set(speed1 / 2);
        let speed2 = bmp.max_speed_get();
        assert!(speed2 < speed1);

        // We should be able to turn off power
        // let duration = Duration::from_secs(1);
        // sleep(duration);
        // assert!(bmp.set_power(false).unwrap());

        // // Target voltage should be OFF after setting power
        // assert_eq!(bmp.target_voltage(), "0.0V");
    }
}
// bool platform_buffer_write(const void *data, size_t size);
// int platform_buffer_read(void *data, size_t size);

// bool remote_swd_init(void);
// bool remote_jtag_init(void);
// bool remote_target_get_power(void);
// const char *remote_target_voltage(void);
// bool remote_target_set_power(bool power);
// void remote_nrst_set_val(bool assert);
// bool remote_nrst_get_val(void);
// void remote_max_frequency_set(uint32_t freq);
// uint32_t remote_max_frequency_get(void);
// void remote_target_clk_output_enable(bool enable);

// void remote_adiv5_dp_init(adiv5_debug_port_s *dp);
// void remote_add_jtag_dev(uint32_t dev_index, const jtag_dev_s *jtag_dev);

// uint64_t remote_decode_response(const char *response, size_t digits);
