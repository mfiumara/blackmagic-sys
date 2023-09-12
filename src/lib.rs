#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    // Import necessary modules and functions
    use std::ffi::CString;

    use super::platform_init;
    use super::remote_hex_string_to_num; // Adjust the import path as needed
    use super::remote_init;
    use super::serial_open;

    #[test]
    fn test_empty_string() {
        let input = CString::new("").expect("CString conversion failed");
        let result = unsafe { remote_hex_string_to_num(0, input.as_ptr()) };
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_digit() {
        let input = CString::new("1").expect("CString conversion failed");
        let result = unsafe { remote_hex_string_to_num(1, input.as_ptr()) };
        assert_eq!(result, 1);
    }

    #[test]
    fn test_multiple_digits() {
        let input = CString::new("123456789ABCDEF").expect("CString conversion failed");
        let result = unsafe { remote_hex_string_to_num(15, input.as_ptr()) };
        assert_eq!(result, 0x123456789ABCDEF);
    }

    #[test]
    fn test_remote_init() {
        unsafe {
            let result = remote_init(true);
            assert_eq!(result, false);
        };
    }
}
// bool platform_buffer_write(const void *data, size_t size);
// int platform_buffer_read(void *data, size_t size);

// bool remote_init(bool power_up);
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
