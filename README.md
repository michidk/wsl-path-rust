# wslpath2
[![MIT License](https://img.shields.io/crates/l/wslpath2)](https://choosealicense.com/licenses/mit/) [![Crates.io](https://img.shields.io/crates/v/wslpath2)](https://crates.io/crates/wslpath2) [![rust docs](https://docs.rs/wslpath2/badge.svg)](https://docs.rs/wslpath2/latest/wslpath2/) [![Continuous integration](https://github.com/michidk/wslpath2/workflows/Continuous%20Integration/badge.svg)](https://github.com/michidk/wslpath2/actions)

You can use `wslpath2` to convert a WSL Path to a Windows Path and vice versa.
Internally it calls the `wslpath` utility, which is a Linux-based utility created by Microsoft to convert Windows and Linux paths. This is called through the WSL command, so that it works on Linux (from within the WSL) as well on Windows.

## About this fork

This is a fork of [wslpath](https://github.com/pratikpc/wsl-path-rust) by pratikpc with the following changes:

- Uses Rust 2021 edition
- Adds settings enum to closely map to `wslpath.exe`'s command line arguments
- Updated to the newest version of `wslpath.exe`
- Error handling is improved

## Usage

The main function is `convert` which has the following signature:

```rust
pub fn convert(
    path: &str,
    distro: Option<&str>,
    options: Conversion,
    force_absolute_path: bool,
) -> Result<String, Box<dyn std::error::Error>>
```

Usage example:

```rust
    // Convert WSL path to Windows
    let path = convert("/mnt/c/Users", None, Conversion::WslToWindows, false).unwrap();
    println!("WSL Path converted to Windows is {}", path);
    // OUTPUT is WSL Path converted to Windows is C:\Users

    // Convert Windows path to WSL
    let path = convert("C:\\Users", None, Conversion::WindowsToWsl, false).unwrap();
    println!("Windows Path converted to WSL is {}", path);
    // OUTPUT is Windows Path converted to WSL is /mnt/c/Users
```

Also, see the [examples](examples) folder for more examples.
