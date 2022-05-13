use std::{ffi::OsStr, os::windows::prelude::OsStrExt};

use windows::core::{PCWSTR, PWSTR};

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
