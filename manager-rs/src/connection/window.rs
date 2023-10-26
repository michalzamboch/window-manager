#![allow(dead_code)]

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn to_wstring(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_DESTROY {
        PostQuitMessage(0);
        return 0;
    }

    return DefWindowProcW(hwnd, msg, wparam, lparam);
}

pub fn create_mock_window() {
    unsafe {
        let wc = get_new_winclass();

        if RegisterClassExW(&wc) == 0 {
            panic!("RegisterClassEx failed");
        }

        let hwnd = CreateWindowExW(
            0,
            wc.lpszClassName,
            to_wstring("Rust Window").as_ptr(),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            640,
            480,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            wc.hInstance,
            std::ptr::null_mut(),
        );

        if hwnd == std::ptr::null_mut() {
            panic!("CreateWindowEx failed");
        }

        ShowWindow(hwnd, SW_SHOW);

        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        loop {
            let res = GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);
            if res == 0 || res == -1 {
                break;
            }

            DispatchMessageW(&msg);
        }
    }
}

unsafe fn get_new_winclass() -> WNDCLASSEXW {
    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
        style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: GetModuleHandleW(std::ptr::null_mut()) as HINSTANCE,
        hIcon: std::ptr::null_mut(),
        hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
        hbrBackground: GetStockObject(WHITE_BRUSH as i32) as HBRUSH,
        lpszMenuName: std::ptr::null_mut(),
        lpszClassName: to_wstring("rust_window_class").as_ptr(),
        hIconSm: std::ptr::null_mut(),
    };

    wc
}
