use std::io;

use crate::util::AsPCWSTR;
use windows::{
    core::PCWSTR,
    Win32::{Foundation::HWND, UI::Shell},
};

pub enum ShowWindow {
    Hide,
    ShowNormal,
    Normal,
    ShowMinimized,
    ShowMaximized,
    Maximize,
    ShowNoActivate,
    Show,
    Minimize,
    ShowMinNoActive,
    ShowNA,
    Restore,
    ShowDefault,
    ForceMinimize,
}

impl ShowWindow {
    fn as_i32(&self) -> i32 {
        match self {
            ShowWindow::Hide => 0,
            ShowWindow::ShowNormal | ShowWindow::Normal => 1,
            ShowWindow::ShowMinimized => 2,
            ShowWindow::ShowMaximized | ShowWindow::Maximize => 3,
            ShowWindow::ShowNoActivate => 4,
            ShowWindow::Show => 5,
            ShowWindow::Minimize => 6,
            ShowWindow::ShowMinNoActive => 7,
            ShowWindow::ShowNA => 8,
            ShowWindow::Restore => 9,
            ShowWindow::ShowDefault => 10,
            ShowWindow::ForceMinimize => 11,
        }
    }
}

pub fn ShellExecute(
    hwnd: HWND,
    operation: Option<&str>,
    file: impl AsRef<str>,
    parameters: Option<&str>,
    directory: Option<&str>,
    show_cmd: ShowWindow,
) -> Result<(), io::Error> {
    let result = unsafe {
        Shell::ShellExecuteW(
            hwnd,
            operation.map_or(PCWSTR(std::ptr::null_mut()), |s| s.as_pcwstr()),
            file.as_ref().as_pcwstr(),
            parameters.map_or(PCWSTR(std::ptr::null_mut()), |s| s.as_pcwstr()),
            directory.map_or(PCWSTR(std::ptr::null_mut()), |s| s.as_pcwstr()),
            show_cmd.as_i32(),
        )
    };

    if result.is_invalid() {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
