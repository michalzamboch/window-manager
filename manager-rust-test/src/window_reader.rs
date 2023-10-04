#[cfg(windows)] extern crate winapi;
use std::io::Error;

use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use winapi::shared::windef::*;

#[cfg(windows)]
pub fn print_message(msg: &str) -> Result<i32, Error> {
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    
    match ret {
        0 => Err(Error::last_os_error()),
        _ => Ok(ret)  
    }
}

#[cfg(not(windows))]
pub fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}


type Fn_HWND_LPARAM_BOOL = unsafe extern "system" fn(hwnd: HWND, l_param: LPARAM) -> BOOL; 

pub unsafe extern "system" fn enum_hwnd(hwnd: HWND, l_param: LPARAM) -> BOOL {
    println!("{:?}", hwnd);
    
    let mut param = l_param as *mut Vec<HWND>;
    //(*param).push(hwnd);

    return TRUE;
}

pub fn get_available_HWNDs() {
    let mut hwnds: Vec<HWND> = Vec::new();
    let mut x: Vec<HWND> = Vec::new();

    
    unsafe {
        let function = Some(enum_hwnd as Fn_HWND_LPARAM_BOOL);
        EnumWindows(function, hwnds.as_mut_ptr() as LPARAM);
    }
}
