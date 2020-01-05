#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate winapi;

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

static mut RUNNING: bool = true;

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE | WM_DESTROY => {
            RUNNING = false;
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn wstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

unsafe fn run() {
    let mut window_class = std::mem::zeroed::<WNDCLASSW>();
    window_class.style = CS_HREDRAW | CS_VREDRAW;
    window_class.lpfnWndProc = Some(window_proc);
    let class_name = wstr("GAME_WINDOW_CLASS");
    window_class.lpszClassName = class_name.as_ptr();

    RegisterClassW(&mut window_class);

    let window_name = wstr("Breakout");
    let hwnd = CreateWindowExW(
        0,
        window_class.lpszClassName,
        window_name.as_ptr(),
        WS_VISIBLE | WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        1280,
        720,
        0 as HWND,
        0 as HMENU,
        0 as HINSTANCE,
        0 as LPVOID,
    );

    while RUNNING {
        let mut msg = std::mem::MaybeUninit::<MSG>::uninit();
        while PeekMessageW(msg.as_mut_ptr(), hwnd, 0 as UINT, 0 as UINT, PM_REMOVE) != 0 {
            let msg = msg.assume_init();
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn main() {
    unsafe {
        run();
    }
}
