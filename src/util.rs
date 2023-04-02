use std::{ffi::OsStr, ops::Deref, os::windows::prelude::OsStrExt, string::FromUtf16Error};

use windows::{
    core::{PCWSTR, PWSTR},
    Win32::Foundation::{CloseHandle, HANDLE},
};

pub trait AsPWSTR {
    fn as_pwstr(&self) -> PWSTR;
}

impl AsPWSTR for String {
    fn as_pwstr(&self) -> PWSTR {
        PWSTR(
            OsStr::new(self)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_mut_ptr(),
        )
    }
}

impl AsPWSTR for &str {
    fn as_pwstr(&self) -> PWSTR {
        PWSTR(
            OsStr::new(self)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_mut_ptr(),
        )
    }
}

pub trait AsPCWSTR {
    fn as_pcwstr(&self) -> PCWSTR;
}

impl AsPCWSTR for String {
    fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR(
            OsStr::new(self)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_mut_ptr(),
        )
    }
}

impl AsPCWSTR for &str {
    fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR(
            OsStr::new(self)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_mut_ptr(),
        )
    }
}

pub trait Close {
    fn close(&self);
}

impl Close for HANDLE {
    fn close(&self) {
        unsafe { CloseHandle(self) };
    }
}

#[derive(Clone, Debug)]
pub struct AutoClosing<T: Close + Clone + std::fmt::Debug>(T);

impl<T: Close + Clone + std::fmt::Debug> From<T> for AutoClosing<T> {
    fn from(item: T) -> Self {
        Self(item)
    }
}

impl<T: Close + Clone + std::fmt::Debug> Drop for AutoClosing<T> {
    fn drop(&mut self) {
        self.0.close();
    }
}

impl<T: Close + Clone + std::fmt::Debug> Deref for AutoClosing<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn get_string(data: &[u16]) -> Result<String, FromUtf16Error> {
    let mut end = 0;

    for (i, &v) in data.iter().enumerate() {
        if v == 0 {
            end = i;
            break;
        }
    }

    String::from_utf16(&data[0..end])
}
