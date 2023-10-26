#![allow(dead_code)]

use winapi::shared::{
    minwindef::{BOOL, LPARAM},
    windef::HWND,
};

pub type FnHwndLParamBool = unsafe extern "system" fn(hwnd: HWND, l_param: LPARAM) -> BOOL;