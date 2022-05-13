use std::ffi::c_void;
use std::io;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug;

pub trait ReadProcessMemory {
    fn read_process_memory_into(
        &self,
        base_address: *const c_void,
        data: &mut [u8],
    ) -> Result<usize, io::Error>;

    fn read_process_memory(
        &self,
        base_address: *const c_void,
        size: usize,
    ) -> Result<usize, io::Error> {
        let mut data = vec![0u8; size];
        self.read_process_memory_into(base_address, data.as_mut())
    }
}

impl ReadProcessMemory for HANDLE {
    fn read_process_memory_into(
        &self,
        base_address: *const c_void,
        data: &mut [u8],
    ) -> Result<usize, io::Error> {
        let mut bytes_read = 0usize;

        let result = unsafe {
            Debug::ReadProcessMemory(
                self,
                base_address,
                data.as_mut_ptr() as *mut _,
                data.len(),
                &mut bytes_read as *mut _,
            )
        };

        if !result.as_bool() {
            return Err(io::Error::last_os_error());
        }

        Ok(bytes_read)
    }
}
