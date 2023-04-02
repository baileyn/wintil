use crate::{
    util::{self, AutoClosing},
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::string::FromUtf16Error;
use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::ToolHelp::{
            MODULEENTRY32W, PROCESSENTRY32W, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
            TH32CS_SNAPPROCESS,
        },
        Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub pid: u32,
    pub num_threads: u32,
    pub parent_pid: u32,
    pub base_thread_priority: i32,
    pub exe_file: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub pid: u32,
    pub base_addr: usize,
    pub size: usize,
    pub name: String,
    pub path: String,
}

pub type ProcessEntry = PROCESSENTRY32W;
pub type ModuleEntry = MODULEENTRY32W;

impl TryFrom<&ProcessEntry> for Process {
    type Error = FromUtf16Error;

    fn try_from(pe: &ProcessEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            pid: pe.th32ProcessID,
            num_threads: pe.cntThreads,
            parent_pid: pe.th32ParentProcessID,
            base_thread_priority: pe.pcPriClassBase,
            exe_file: util::get_string(&pe.szExeFile)?,
        })
    }
}

impl TryFrom<&ModuleEntry> for Module {
    type Error = FromUtf16Error;

    fn try_from(pe: &ModuleEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            pid: pe.th32ProcessID,
            base_addr: pe.modBaseAddr as usize,
            size: pe.dwSize as usize,
            name: util::get_string(&pe.szModule)?,
            path: util::get_string(&pe.szExePath)?,
        })
    }
}

impl Process {
    pub fn for_name(name: impl AsRef<str>) -> Result<Vec<Process>, io::Error> {
        Ok(Self::all()?
            .into_iter()
            .filter(|p| p.exe_file == name.as_ref())
            .collect())
    }

    pub fn all() -> Result<Vec<Process>, io::Error> {
        let mut processes = Vec::new();
        let mut pe = ProcessEntry::default();
        pe.dwSize = std::mem::size_of::<ProcessEntry>() as u32;

        let th32 = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        if th32.is_invalid() {
            return Err(std::io::Error::last_os_error());
        }

        if Process32First(th32, &mut pe) {
            loop {
                let process = Process::try_from(&pe).unwrap();
                processes.push(process);

                if !Process32Next(th32, &mut pe) {
                    break;
                }
            }
        }

        Ok(processes)
    }

    pub fn base_addr(&self) -> Result<usize, io::Error> {
        let modules = self.modules()?;
        let mut modules = modules.iter().filter(|&m| m.name == self.exe_file);

        if let Some(next) = modules.next() {
            Ok(next.base_addr)
        } else {
            Err(io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                "unable to find base address",
            ))
        }
    }

    pub fn modules(&self) -> Result<Vec<Module>, io::Error> {
        let mut modules = Vec::new();
        let mut me = ModuleEntry::default();
        me.dwSize = std::mem::size_of::<ModuleEntry>() as u32;

        let th32 = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.pid)?;
        if th32.is_invalid() {
            return Err(io::Error::last_os_error());
        }

        if Module32First(th32, &mut me) {
            loop {
                let module = Module::try_from(&me).unwrap();
                modules.push(module);

                if !Module32Next(th32, &mut me) {
                    break;
                }
            }
        }

        Ok(modules)
    }

    pub fn open(
        &self,
        desired_access: PROCESS_ACCESS_RIGHTS,
        inherit_handle: bool,
    ) -> Result<AutoClosing<HANDLE>, io::Error> {
        let handle = unsafe { OpenProcess(desired_access, inherit_handle, self.pid) }?;

        Ok(handle.into())
    }
}
