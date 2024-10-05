use core::ffi::{c_char, c_int, c_void};

use alloc::{ffi::CString, string::ToString, vec::Vec};

use cfg_if::cfg_if;

use crate::ConvertLossyError;

#[allow(non_camel_case_types)]
pub type iconv_t = *mut c_void;

cfg_if! {
    if #[cfg(all(
        feature = "libc-iconv",
        any(
            all(target_env = "gnu", target_os = "linux"),
            target_os = "hurd",
            target_vendor = "apple"
        )
    ))] {
        extern "C" {
            pub fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t;
            pub fn iconv(
                cd: iconv_t,
                inbuf: *mut *const c_char,
                inbytesleft: *mut usize,
                outbuf: *mut *mut c_char,
                outbytesleft: *mut usize,
            ) -> usize;
            pub fn iconv_close(cd: iconv_t) -> c_int;
        }
    } else if #[cfg(not(feature = "libc-iconv"))] {
        #[link(name = "iconv")]
        extern "C" {
            #[link_name = "libiconv_open"]
            pub fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t;
            #[link_name = "libiconv"]
            pub fn iconv(
                cd: iconv_t,
                inbuf: *mut *const c_char,
                inbytesleft: *mut usize,
                outbuf: *mut *mut c_char,
                outbytesleft: *mut usize,
            ) -> usize;
            #[link_name = "libiconv_close"]
            pub fn iconv_close(cd: iconv_t) -> c_int;
        }
    } else {
        compile_error!(
            "non-GNU libc is not supported due to feature differences. Please consider using libiconv."
        );
    }
}

#[repr(transparent)]
pub struct LossyIconv(iconv_t);

unsafe impl Send for LossyIconv {}

impl LossyIconv {
    pub fn new(from_encoding: &str, to_encoding: &str) -> Result<Self, ConvertLossyError> {
        let from_encoding =
            CString::new(from_encoding).map_err(|_| ConvertLossyError::UnknownConversion)?;
        let to_encoding = to_encoding.to_string()
            + if to_encoding.contains("//") {
                "\0"
            } else {
                "//IGNORE\0"
            };
        let to_encoding = CString::from_vec_with_nul(to_encoding.into_bytes())
            .map_err(|_| ConvertLossyError::UnknownConversion)?;
        unsafe {
            let cd = iconv_open(to_encoding.as_ptr(), from_encoding.as_ptr());
            if cd as isize == -1 {
                return Err(ConvertLossyError::UnknownConversion);
            }
            Ok(Self(cd))
        }
    }

    pub fn convert(&self, mut input: &[u8]) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::with_capacity(input.len());
        unsafe {
            loop {
                let mut inbuf_ptr = input.as_ptr() as *const c_char;
                let mut inlen = input.len();
                let outbuf = output.spare_capacity_mut();
                let mut outbuf_ptr = outbuf.as_mut_ptr();
                let mut outlen = outbuf.len();
                let res = iconv(
                    self.0,
                    &mut inbuf_ptr as *mut _,
                    &mut inlen,
                    &mut outbuf_ptr as *mut _ as *mut *mut c_char,
                    &mut outlen,
                );
                let is_last_error_e2big = is_last_error_e2big();
                let new_len = outbuf.len() - outlen + output.len();
                output.set_len(new_len);
                if res as isize == -1 {
                    if !is_last_error_e2big {
                        panic!("iconv error");
                    }
                    // E2BIG
                    input = &input[input.len() - inlen..];
                    output.reserve(output.capacity() * 2 - output.len());
                } else {
                    output.shrink_to_fit();
                    break output;
                }
            }
        }
    }
}

#[cfg(windows)]
fn is_last_error_e2big() -> bool {
    extern "C" {
        fn _errno() -> *mut c_int;
    }
    unsafe { *_errno() == 7 }
}

#[cfg(not(windows))]
fn is_last_error_e2big() -> bool {
    dbg!(std::io::Error::last_os_error().raw_os_error()) == Some(7)
}

impl Drop for LossyIconv {
    fn drop(&mut self) {
        unsafe { iconv_close(self.0) };
    }
}
