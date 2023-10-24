#![allow(dead_code)]

use winapi::shared::windef::HWND;

use crate::connection::{hwnd::*, title::*};

pub fn list_all_hwnds() {
    let hwnds = get_window_all_hwnds();
    list_hwnds(hwnds);
}

fn list_hwnds(hwnds: Vec<HWND>) {
    for hwnd in hwnds {
        println!("{:?}", hwnd);
    }
}

pub fn list_all_window() {
    let titles = get_window_all_titles();
    list_windows(titles);
}

fn list_windows(window_titles: Vec<String>) {
    for title in window_titles {
        println!("{}", title);
    }
}

pub fn list_wisible_windows() {
    let titles = get_windows_visible_titles();

    titles.into_iter().for_each(|title| println!("{}", title));
}
