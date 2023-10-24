#![allow(dead_code)]

use winapi::{
    shared::windef::{HWND, POINT, RECT},
    um::winuser::{GetWindowPlacement, SW_SHOWNORMAL, WINDOWPLACEMENT},
};

pub fn get_window_placement(hwnd: &HWND) -> WINDOWPLACEMENT {
    let mut wp = get_basic_window_placement();

    unsafe {
        GetWindowPlacement(*hwnd, &mut wp);
    }

    wp
}

fn get_basic_window_placement() -> WINDOWPLACEMENT {
    WINDOWPLACEMENT {
        length: 0,
        flags: 0,
        showCmd: SW_SHOWNORMAL as u32,
        ptMinPosition: POINT { x: 0, y: 0 },
        ptMaxPosition: POINT { x: 0, y: 0 },
        rcNormalPosition: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
    }
}

pub fn set_coords(wp: &mut WINDOWPLACEMENT, x: i32, y: i32, w: i32, h: i32) {
    wp.showCmd = SW_SHOWNORMAL as u32;
    wp.ptMinPosition.x = 100;
    wp.ptMinPosition.y = 100;
    wp.ptMaxPosition.x = 800;
    wp.ptMaxPosition.y = 600;
    wp.rcNormalPosition.left = x;
    wp.rcNormalPosition.top = y;
    wp.rcNormalPosition.right = x + w;
    wp.rcNormalPosition.bottom = y + h;
}
