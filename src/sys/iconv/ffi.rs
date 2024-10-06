use core::ffi::{c_char, c_int, c_void};
use core::ptr::null;

use alloc::{ffi::CString, string::ToString, vec::Vec};

use cfg_if::cfg_if;

use crate::{ConvertError, ConvertLossyError};

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
        #[cfg_attr(target_vendor = "apple", link(name = "iconv"))]
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
pub struct IconvHandle(iconv_t);
#[repr(transparent)]
pub struct LossyIconv(IconvHandle);
#[repr(transparent)]
pub struct Iconv(IconvHandle);

unsafe impl Send for IconvHandle {}
unsafe impl Sync for IconvHandle {}

impl IconvHandle {
    fn new(from_encoding: &str, to_encoding: &str, lossy: bool) -> Result<Self, ConvertLossyError> {
        let from_encoding =
            CString::new(from_encoding).map_err(|_| ConvertLossyError::UnknownConversion)?;
        let to_encoding = to_encoding.to_string()
            + if to_encoding.contains("//") || !lossy {
                "\0"
            } else {
                "//IGNORE//TRANSLIT\0"
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

    fn convert(&mut self, mut input: &[u8]) -> Result<Vec<u8>, (Vec<u8>, ConvertError)> {
        let mut output: Vec<u8> = Vec::with_capacity(input.len());
        unsafe {
            loop {
                let mut inlen = input.len();
                let mut inbuf_ptr = if inlen == 0 {
                    null()
                } else {
                    input.as_ptr() as *const c_char
                };
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
                input = &input[input.len() - inlen..];
                if res as isize == -1 && is_last_error_e2big {
                    // E2BIG
                    output.reserve(output.capacity() * 2 - output.len());
                    continue;
                }
                // glibc: even when //IGNORE is used, iconv may still return EINVAL.
                // Ignore any errors for a best-effort lossy conversion.
                if res as isize == -1 {
                    output.shrink_to_fit();
                    break Err((output, ConvertError::InvalidInput));
                }
                // When inbuf_ptr is null, it means iconv has been called once with an empty input,
                // which is essentially a flush being done.
                if inbuf_ptr.is_null() {
                    output.shrink_to_fit();
                    break Ok(output);
                }
            }
        }
    }
}

impl LossyIconv {
    pub fn new(from_encoding: &str, to_encoding: &str) -> Result<Self, ConvertLossyError> {
        IconvHandle::new(from_encoding, to_encoding, true).map(Self)
    }

    pub fn convert(&mut self, input: &[u8]) -> Vec<u8> {
        self.0.convert(input).unwrap_or_else(|(buf, _)| buf)
    }
}

impl Iconv {
    pub fn new(from_encoding: &str, to_encoding: &str) -> Result<Self, ConvertLossyError> {
        IconvHandle::new(from_encoding, to_encoding, false).map(Self)
    }

    pub fn convert(&mut self, input: &[u8]) -> Result<Vec<u8>, ConvertError> {
        self.0.convert(input).map_err(|(_, e)| e)
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
    std::io::Error::last_os_error().raw_os_error() == Some(7)
}

impl Drop for IconvHandle {
    fn drop(&mut self) {
        unsafe { iconv_close(self.0) };
    }
}
