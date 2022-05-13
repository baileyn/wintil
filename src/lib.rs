#![allow(non_snake_case)]

#[cfg(feature = "Win32_System_Diagnostics_ToolHelp")]
mod threading;

#[cfg(feature = "Win32_System_Diagnostics_ToolHelp")]
pub use threading::*;
