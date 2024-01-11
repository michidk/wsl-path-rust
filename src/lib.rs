//! Convert Windows paths to WSL paths and vice versa
//!
//! # Example
//!
//! ```
//! use wslpath2::{convert, Conversion};
//!
//! let path = convert("/mnt/c", None, Conversion::WslToWindows, false).unwrap_or_default();
//!
//! assert_eq!(path, "C:\\");
//! ```

#![warn(missing_docs)]

use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Type of conversion to perform
#[derive(Debug)]
pub enum Conversion {
    /// Convert Windows path to WSL path
    WindowsToWsl,
    /// Convert WSL path to Windows path
    WslToWindows,
    /// Convert WSL path to Windows path using Linux style path separators
    WslToWindowsLinuxStyle,
}

/// Convert Paths using the `wslpath`
///
/// # Arguments
///
/// * `path` - The path to convert
/// * `distro` - The distro to use for conversion (when calling from Windows) [optional]
/// * `options` - The type of conversion to perform
/// * `force_absolute_path` - Force the path to be absolute
///
/// # Example
///
/// ```
/// use wslpath2::{convert, Conversion};
///
/// let path = convert("/mnt/c", None, Conversion::WslToWindows, false).unwrap_or_default();
///
/// assert_eq!(path, "C:\\");
/// ```
pub fn convert(
    path: &str,
    distro: Option<&str>,
    options: Conversion,
    force_absolute_path: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut args = Vec::new();

    if let Some(distro) = distro {
        args.push("-d");
        args.push(distro);
    }
    args.push("-e");
    args.push("wslpath");

    // Select path arg
    // Based on this conversion takes place
    args.push(match options {
        Conversion::WindowsToWsl => "-u",
        Conversion::WslToWindows => "-w",
        Conversion::WslToWindowsLinuxStyle => "-m",
    });

    // force absolute path arg
    if force_absolute_path {
        args.push("-a");
    }

    let mut cmd = Command::new("wsl.exe");
    cmd.args(args);
    cmd.arg(path.replace('\\', "\\\\"));

    // Disable window creation on Windows
    //
    // This is necessary to prevent a command prompt window from being shown for a short time,
    // which is likely undesired, especially for GUI applications.
    //
    // The flags are documented here:
    // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#flags
    #[cfg(windows)]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    let output = cmd.output()?;

    let code = output.status.code().unwrap_or(-1);
    if code != 0 {
        return Err(format!("Error getting wslpath: {}", code).into());
    }

    Ok(std::str::from_utf8(&output.stdout)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use crate::{convert, Conversion};

    // These tests may not execute on all machines as they require WSL

    #[test]
    fn test_wsl_to_windows() {
        assert_eq!(
            convert("/mnt/c", None, Conversion::WslToWindows, false).unwrap_or_default(),
            "C:\\"
        );
    }

    #[test]
    fn test_wsl_to_windows_absolute() {
        assert_eq!(
            convert("/mnt/c", None, Conversion::WslToWindows, true).unwrap_or_default(),
            "C:\\"
        );
    }

    #[test]
    fn test_wsl_to_windows_linux_style() {
        assert_eq!(
            convert("/mnt/c", None, Conversion::WslToWindowsLinuxStyle, false).unwrap_or_default(),
            "C:/"
        );
    }

    #[test]
    fn test_wsl_to_windows_linux_style_absolute() {
        assert_eq!(
            convert("/mnt/c", None, Conversion::WslToWindowsLinuxStyle, true).unwrap_or_default(),
            "C:/"
        );
    }

    #[test]
    fn test_windows_to_wsl() {
        assert_eq!(
            convert("C:/", None, Conversion::WindowsToWsl, false).unwrap_or_default(),
            "/mnt/c/"
        );
    }

    #[test]
    fn test_windows_to_wsl_absolute() {
        assert_eq!(
            convert("C:/", None, Conversion::WindowsToWsl, true).unwrap_or_default(),
            "/mnt/c/"
        );
    }
}
