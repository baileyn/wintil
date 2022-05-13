use std::io;

use windows::Win32::{Foundation::HANDLE, System::Diagnostics::ToolHelp};

type ProcessEntry = ToolHelp::PROCESSENTRY32W;
type ModuleEntry = ToolHelp::MODULEENTRY32W;

pub fn CreateToolhelp32Snapshot(
    flags: ToolHelp::CREATE_TOOLHELP_SNAPSHOT_FLAGS,
    pid: u32,
) -> Result<HANDLE, io::Error> {
    Ok(unsafe { ToolHelp::CreateToolhelp32Snapshot(flags, pid) }?)
}

pub fn Process32First(snapshot: HANDLE, pe: &mut ProcessEntry) -> bool {
    unsafe { ToolHelp::Process32FirstW(snapshot, pe as *mut _) }.as_bool()
}

pub fn Process32Next(snapshot: HANDLE, pe: &mut ProcessEntry) -> bool {
    unsafe { ToolHelp::Process32NextW(snapshot, pe as *mut _) }.as_bool()
}

pub fn Module32First(snapshot: HANDLE, me: &mut ModuleEntry) -> bool {
    unsafe { ToolHelp::Module32FirstW(snapshot, me as *mut _) }.as_bool()
}

pub fn Module32Next(snapshot: HANDLE, me: &mut ModuleEntry) -> bool {
    unsafe { ToolHelp::Module32NextW(snapshot, me as *mut _) }.as_bool()
}
