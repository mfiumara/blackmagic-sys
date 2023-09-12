#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::ptr;

// use {bmda_cli_mode_BMP_MODE_DEBUG, bmda_cli_options, bmp_scan_mode_BMP_SCAN_SWD, serial_open};

// Define your custom error type (Err)
#[derive(Debug)]
pub struct BmpError {
    message: String,
}

impl fmt::Display for BmpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BmpError {}
/// A result type with the error hardwired to [`BmpError`].
pub type Result<T> = std::result::Result<T, BmpError>;
pub struct Bmp {
    serial: String,
}

impl Bmp {
    pub fn open_by_serial(serial: &str) -> Result<Self> {
        let input = CString::new(serial).expect("CString conversion failed");
        let options = bmda_cli_options {
            opt_target_dev: 1,
            opt_flash_size: 0xffffffff,
            opt_flash_start: 0xffffffff,
            opt_max_swj_frequency: 4000000,
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
            return Err(BmpError {
                message: "Could not open serial device".to_string(),
            });
        }
        let bmp = Bmp {
            serial: serial.to_string(),
        };

        let result = unsafe { remote_init(true) };
        if result == false {
            return Err(BmpError {
                message: "Could not initialize device".to_string(),
            });
        }

        let ret: Result<Bmp> = Ok(bmp);
        return ret;
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
    fn test_bmp_open() {
        let bmp = super::Bmp::open_by_serial("98B72495");
        match bmp {
            Ok(bmp) => {}
            Err(e) => {
                assert!(false, "open_by_serial returned error");
            }
        }
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
