use std::{error::Error, result::Result as StdResult};

use winapi::shared::{
    minwindef::{BOOL, LPARAM},
    windef::HWND,
};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

type FN_HWND_LPARAM_BOOL = unsafe extern "system" fn(hwnd: HWND, l_param: LPARAM) -> BOOL;

pub trait ConnectionTrait: Sized {
    fn new() -> Result<Self>;
    fn window_titles(&self) -> Result<Vec<String>>;
}
